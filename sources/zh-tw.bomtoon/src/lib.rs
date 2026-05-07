#![no_std]

mod net;
mod response;

use {
	aidoku::{
		AidokuError, AlternateCoverProvider, Chapter, FilterValue, HashMap, Manga, MangaPageResult,
		Page, Result, Source, WebLoginHandler,
		alloc::{String, Vec, format},
		bail,
		imports::std::send_partial_result,
		register_source,
	},
	arrayvec::ArrayString,
	core::cell::Cell,
	net::Url,
	response::{Ranking, Search, Session, UpdateManga as _},
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
		let index = page
			.saturating_sub(1)
			.try_into()
			.map_err(AidokuError::message)?;
		let next_pagination = (index != 0).then(|| self.next_pagination.get());
		#[expect(clippy::shadow_reuse, reason = "guarded")]
		let url = if let Some(query) = query.as_deref() {
			use net::search::{Method, SortBy, Type};

			let (r#type, search_text) = query
				.strip_prefix('#')
				.map_or((Type::All, query), |tag| (Type::Tag, tag));

			Url::search(
				r#type,
				search_text,
				index,
				next_pagination,
				Method::Input,
				SortBy::Popular,
			)
		} else {
			Url::from_filters(&filters, index, next_pagination)?
		};
		let mut res = url.request()?.send()?;
		let result = if url.is_search() {
			let search = res.get_json::<Search>()?;
			self.next_pagination.set(search.next_pagination());

			search.manga_page_result()
		} else {
			res.get_json::<Ranking>()?.manga_page_result()
		};
		Ok(result)
	}

	fn get_manga_update(
		&self,
		mut manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		let mut res = Url::manga(&manga.key).request()?.send()?;
		let updated_manga = res.get_json::<response::Manga>()?;
		if needs_details {
			manga.update_details(&updated_manga);

			if needs_chapters {
				send_partial_result(&manga);
			} else {
				return Ok(manga);
			}
		}

		manga.update_chapters(&updated_manga);
		Ok(manga)
	}

	fn get_page_list(&self, manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		todo!()
	}
}

impl AlternateCoverProvider for Bomtoon {
	fn get_alternate_covers(&self, manga: Manga) -> Result<Vec<String>> {
		let covers = Url::manga(&manga.key)
			.request()?
			.send()?
			.get_json::<response::Manga>()?
			.covers();
		Ok(covers)
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

register_source!(Bomtoon, AlternateCoverProvider, WebLoginHandler);
