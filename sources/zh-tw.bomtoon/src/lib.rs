#![no_std]

mod net;
mod response;

use {
	aidoku::{
		AidokuError, Chapter, FilterValue, HashMap, Manga, MangaPageResult, Page, Result, Source,
		WebLoginHandler,
		alloc::{String, Vec, format},
		bail, register_source,
	},
	arrayvec::ArrayString,
	core::cell::Cell,
	net::Url,
	response::Session,
};

struct Bomtoon {
	next_pagination: Cell<ArrayString<31>>,
}

impl Source for Bomtoon {
	fn new() -> Self {
		Self {
			next_pagination: Cell::new(ArrayString::new_const()),
		}
	}

	fn get_search_manga_list(
		&self,
		query: Option<String>,
		page: i32,
		filters: Vec<FilterValue>,
	) -> Result<MangaPageResult> {
		#[expect(clippy::shadow_reuse, reason = "guarded")]
		let url = if let Some(query) = query.as_deref() {
			use net::search::{SortBy, Type};

			let (r#type, search_text) = query
				.strip_prefix('#')
				.map_or((Type::All, query), |tag| (Type::Tag, tag));

			let index = page
				.saturating_sub(1)
				.try_into()
				.map_err(AidokuError::message)?;

			let next_pagination = (index != 0).then(|| self.next_pagination.get());

			Url::search(r#type, search_text, index, next_pagination, SortBy::Popular)
		} else {
			todo!()
		};
		todo!()
	}

	fn get_manga_update(
		&self,
		manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		todo!()
	}

	fn get_page_list(&self, manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		todo!()
	}
}

impl WebLoginHandler for Bomtoon {
	fn handle_web_login(&self, key: String, cookies: HashMap<String, String>) -> Result<bool> {
		if key != "login" {
			bail!("invalid login key: {key}");
		}
		let Some(session_token) = cookies.get("__Secure-next-auth.session-token") else {
			return Ok(false);
		};
		Url::Session
			.request()?
			.header(
				"Cookie",
				&format!("__Secure-next-auth.session-token={session_token}"),
			)
			.json_owned::<Session>()?
			.refresh_access_token();
		Ok(true)
	}
}

register_source!(Bomtoon, WebLoginHandler);
