#![expect(clippy::min_ident_chars, reason = "query key")]

pub mod ranking;
pub mod search;

use {
	crate::response::{AccessToken, NextData as _},
	aidoku::{
		AidokuError, FilterValue,
		alloc::{String, Vec, collections::BTreeSet, fmt::Write as _},
		bail, error,
		helpers::uri::QueryParameters,
		imports::{
			defaults::{defaults_get, defaults_set_data},
			net::Request,
		},
		serde::{Deserialize, Serialize, Serializer},
	},
	arrayvec::ArrayString,
	strum::{AsRefStr, EnumIs},
};

#[derive(EnumIs, Serialize)]
#[serde(untagged)]
pub enum Url<'a> {
	Base,
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
		#[serde(serialize_with = "comma_join")]
		contents_thumbnail_type: BTreeSet<ThumbnailType>,
		search_method: search::Method,
		sort: search::SortBy,
	},
	#[serde(rename_all = "camelCase")]
	Ranking {
		adult_toggle: bool,
		#[serde(serialize_with = "comma_join")]
		contents_thumbnail_type: BTreeSet<ThumbnailType>,
		ranking_type: ranking::Type,
		#[serde(skip_serializing_if = "Option::is_none")]
		is_global_release: Option<bool>,
		#[serde(skip_serializing_if = "Option::is_none")]
		badge_complete: Option<bool>,
		#[serde(
			skip_serializing_if = "Option::is_none",
			serialize_with = "option_comma_join"
		)]
		contents_free_filter: Option<BTreeSet<&'a str>>,
		#[serde(
			skip_serializing_if = "Option::is_none",
			serialize_with = "option_comma_join"
		)]
		genres: Option<BTreeSet<&'a str>>,
		#[serde(
			skip_serializing_if = "Option::is_none",
			serialize_with = "option_comma_join"
		)]
		genres_internal_name: Option<BTreeSet<&'a str>>,
	},
	MangaPage {
		key: &'a str,
	},
	#[serde(rename_all = "camelCase")]
	Manga {
		#[serde(skip)]
		key: &'a str,

		is_not_login_adult: bool,
		is_porch: bool,
	},
	ChapterPage {
		manga_key: &'a str,
		key: &'a str,
	},
	#[serde(rename_all = "camelCase")]
	Chapter {
		alias: &'a str,
		ep_alias: &'a str,
	},
	Main {
		platform: &'static str,
		key: &'static str,
		t: &'static str,
	},
	Event(&'a str),
	Shop,
	Play,
	Pick,
}

impl Url<'_> {
	pub const fn main() -> Self {
		Self::Main {
			platform: "bom",
			key: "comic",
			t: "main",
		}
	}

	pub fn to_string(&self) -> aidoku::Result<String> {
		let mut url = String::from("https://www.bomtoon.tw");
		match *self {
			Self::Base => url.push('/'),
			Self::Session => url.push_str("/api/auth/session"),
			Self::Search { r#type, .. } => {
				let query = QueryParameters::from_data(self)?;
				write!(url, "/api/balcony-api-v2/search/{type}?{query}")
					.map_err(AidokuError::message)?;
			}
			Self::Ranking { .. } => {
				let query = QueryParameters::from_data(self)?;
				write!(
					url,
					"/api/balcony-api-v2/contents/tab/ranking/comic?{query}"
				)
				.map_err(AidokuError::message)?;
			}
			Self::MangaPage { key } => {
				write!(url, "/detail/{key}").map_err(AidokuError::message)?;
			}
			Self::Manga { key, .. } => {
				let query = QueryParameters::from_data(self)?;
				write!(url, "/api/balcony-api-v2/contents/{key}?{query}")
					.map_err(AidokuError::message)?;
			}
			Self::ChapterPage { manga_key, key } => {
				write!(url, "/viewer/{manga_key}/{key}").map_err(AidokuError::message)?;
			}
			Self::Chapter { alias, ep_alias } => {
				let build_id = build_id()?;
				let query = QueryParameters::from_data(self)?;
				write!(
					url,
					"/_next/data/{build_id}/viewer/{alias}/{ep_alias}.json?{query}"
				)
				.map_err(AidokuError::message)?;
			}
			Self::Main { .. } => {
				let build_id = build_id()?;
				let query = QueryParameters::from_data(self)?;
				write!(
					url,
					"/_next/data/{build_id}/main/bom/comic/main.json?{query}"
				)
				.map_err(AidokuError::message)?;
			}
			Self::Event(event) => write!(url, "/event/{event}").map_err(AidokuError::message)?,
			Self::Shop => url.push_str("/shop"),
			Self::Play => url.push_str("/play"),
			Self::Pick => url.push_str("/comic/pick"),
		}
		Ok(url)
	}

	pub fn request(&self) -> aidoku::Result<Request> {
		let url = self.to_string()?;
		let mut request = Request::get(url)?;

		if self.is_search() || self.is_ranking() || self.is_manga() {
			request.set_header("x-balcony-id", "BOMTOON_TW");
		}

		if self.is_manga() && defaults_get::<String>("login").is_some() {
			let token = defaults_get::<AccessToken>("accessToken")
				.ok_or_else(|| {
					error!("default not found or not `AccessToken` for key: `accessToken`")
				})?
				.token()?;
			#[expect(
				clippy::unwrap_used,
				clippy::unwrap_in_result,
				reason = "smaller than capacity"
			)]
			let mut bearer = ArrayString::<535>::from("Bearer ").unwrap();
			bearer.push_str(&token);
			request.set_header("Authorization", &bearer);
		}

		Ok(request)
	}
}

impl<'a> Url<'a> {
	pub const fn manga(key: &'a str) -> Self {
		Self::Manga {
			key,
			is_not_login_adult: false,
			is_porch: false,
		}
	}

	pub fn search(
		r#type: search::Type,
		search_text: &'a str,
		page: u8,
		next_pagination: Option<ArrayString<31>>,
		search_method: search::Method,
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
			contents_thumbnail_type: BTreeSet::from_iter([ThumbnailType::Vertical]),
			search_method,
			sort,
		}
	}

	pub fn from_filters(
		filters: &'a [FilterValue],
		page: u8,
		next_pagination: Option<ArrayString<31>>,
	) -> aidoku::Result<Self> {
		let mut ranking_type = ranking::Type::default();
		let mut is_global_release = false;
		let mut badge_complete = None;
		let mut contents_free_filter = None;
		let mut genres = None;
		let mut genres_internal_name = None;

		for filter in filters {
			#[expect(clippy::match_wildcard_for_single_variants, reason = "intended")]
			match *filter {
				FilterValue::Text { ref id, ref value } => match id.as_str() {
					"author" => {
						return Ok(Self::search(
							search::Type::All,
							value,
							page,
							next_pagination,
							search::Method::Input,
							search::SortBy::Popular,
						));
					}
					_ => bail!("invalid text filter id: `{id}`"),
				},
				FilterValue::Sort { ref id, index, .. } => match id.as_str() {
					"排行" => ranking_type = ranking::Type::from_repr(index).unwrap_or_default(),
					_ => bail!("invalid sort filter id: `{id}`"),
				},
				FilterValue::Check { ref id, value } => match id.as_str() {
					"WorldDrop" => is_global_release = value == 1,
					_ => bail!("invalid check filter id: `{id}`"),
				},
				FilterValue::Select { ref id, ref value } => match id.as_str() {
					"genre" => {
						return Ok(Self::search(
							search::Type::Tag,
							value,
							page,
							next_pagination,
							search::Method::Click,
							search::SortBy::Popular,
						));
					}
					_ => bail!("invalid select filter id: `{id}`"),
				},
				FilterValue::MultiSelect {
					ref id,
					ref included,
					..
				} => match id.as_str() {
					"類型" => {
						let (ids, names) = included
							.iter()
							.filter_map(|genre| genre.split_once('|'))
							.unzip();
						genres = Some(ids);
						genres_internal_name = Some(names);
					}
					"進度" => {
						badge_complete =
							match included.iter().map(AsRef::as_ref).collect::<Vec<_>>()[..] {
								["完結"] => Some(true),
								["連載"] => Some(false),
								_ => None,
							}
					}
					"優惠" => {
						let types = included.iter().map(AsRef::as_ref).collect();
						contents_free_filter = Some(types);
					}
					_ => bail!("invalid multi-select filter id: `{id}`"),
				},
				_ => bail!("invalid filter: `{filter:?}`"),
			}
		}

		Ok(Self::ranking(
			ranking_type,
			is_global_release,
			badge_complete,
			contents_free_filter,
			genres,
			genres_internal_name,
		))
	}

	fn ranking(
		ranking_type: ranking::Type,
		is_global_release: bool,
		badge_complete: Option<bool>,
		contents_free_filter: Option<BTreeSet<&'a str>>,
		genres: Option<BTreeSet<&'a str>>,
		genres_internal_name: Option<BTreeSet<&'a str>>,
	) -> Self {
		Self::Ranking {
			adult_toggle: true,
			contents_thumbnail_type: BTreeSet::from_iter([ThumbnailType::Vertical]),
			ranking_type,
			is_global_release: is_global_release.then_some(true),
			badge_complete,
			contents_free_filter,
			genres,
			genres_internal_name,
		}
	}
}

#[derive(AsRefStr, PartialEq, Eq, PartialOrd, Ord, Deserialize, EnumIs)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThumbnailType {
	Vertical,
	Main,
	Square,
	Detail,
	HorizontalTypeA,
	DetailNonAdult,
	Cover,
	HorizontalTypeB,
	HorizontalTypeC,
	MainNonAdult,
}

fn comma_join<T: AsRef<str>, S: Serializer>(
	items: &BTreeSet<T>,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	items
		.iter()
		.map(AsRef::as_ref)
		.collect::<Vec<_>>()
		.join(",")
		.serialize(serializer)
}

#[expect(clippy::ref_option, reason = "required by `serialize_with`")]
fn option_comma_join<T: AsRef<str>, S: Serializer>(
	items: &Option<BTreeSet<T>>,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	#[expect(clippy::shadow_reuse, reason = "guarded")]
	let Some(items) = items.as_ref() else {
		return serializer.serialize_none();
	};
	comma_join(items, serializer)
}

fn build_id() -> Result<ArrayString<21>, AidokuError> {
	let build_id = if let Some(build_id) = defaults_get::<ArrayString<21>>("buildId") {
		build_id
	} else {
		let build_id = Url::Base.request()?.html()?.next_data()?.build_id();
		defaults_set_data("buildId", build_id);
		build_id
	};
	Ok(build_id)
}
