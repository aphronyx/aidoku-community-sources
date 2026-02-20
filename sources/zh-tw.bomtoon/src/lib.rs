#![no_std]

mod net;
mod response;

use {
	aidoku::{
		Chapter, FilterValue, HashMap, Manga, MangaPageResult, Page, Result, Source,
		WebLoginHandler,
		alloc::{String, Vec, format},
		bail, register_source,
	},
	net::Url,
	response::Session,
};

struct Bomtoon;

impl Source for Bomtoon {
	fn new() -> Self {
		Self
	}

	fn get_search_manga_list(
		&self,
		query: Option<String>,
		page: i32,
		filters: Vec<FilterValue>,
	) -> Result<MangaPageResult> {
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
