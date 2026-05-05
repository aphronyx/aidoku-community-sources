pub mod search;

use {
	aidoku::{
		AidokuError,
		alloc::{String, collections::BTreeSet, fmt::Write as _},
		helpers::uri::QueryParameters,
		imports::net::Request,
		serde::Serialize,
	},
	arrayvec::ArrayString,
	strum::EnumIs,
};

#[derive(EnumIs, Serialize)]
#[serde(untagged)]
pub enum Url<'a> {
	Session,
	#[serde(rename_all = "camelCase")]
	Search {
		#[serde(skip)]
		r#type: search::Type,

		search_text: &'a str,
		is_include_adult: bool,
		page: u8,
		size: u8,
		#[serde(skip_serializing_if = "Option::is_none")]
		next_pagination: Option<ArrayString<31>>,
		is_check_device: bool,
		#[serde(serialize_with = "search::thumbnail_types")]
		contents_thumbnail_type: BTreeSet<search::ThumbnailType>,
		search_method: search::Method,
		sort: search::SortBy,
	},
}

impl Url<'_> {
	pub fn request(&self) -> aidoku::Result<Request> {
		let url = self.to_string()?;
		let request = Request::get(url)?;
		Ok(request)
	}

	fn to_string(&self) -> aidoku::Result<String> {
		let mut url = String::from("https://www.bomtoon.tw");
		match *self {
			Self::Session => url.push_str("/api/auth/session"),
			Self::Search { r#type, .. } => {
				let query = QueryParameters::from_data(self)?;
				write!(url, "/api/balcony-api-v2/search/{type}?{query}")
					.map_err(AidokuError::message)?;
			}
		}
		Ok(url)
	}
}

impl<'a> Url<'a> {
	pub fn search(
		r#type: search::Type,
		search_text: &'a str,
		page: u8,
		next_pagination: Option<ArrayString<31>>,
		sort: search::SortBy,
	) -> Self {
		Self::Search {
			r#type,
			search_text,
			is_include_adult: true,
			page,
			size: 50,
			next_pagination,
			is_check_device: true,
			contents_thumbnail_type: BTreeSet::from_iter([search::ThumbnailType::Detail]),
			search_method: search::Method::Input,
			sort,
		}
	}
}
