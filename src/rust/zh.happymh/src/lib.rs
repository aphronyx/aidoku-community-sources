#![no_std]

extern crate alloc;

mod helper;

use aidoku::{error::Result, prelude::get_manga_list, std::Vec, Filter, Manga, MangaPageResult};
use alloc::string::ToString as _;
use helper::url::Url;

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let referer = Url::Abs { path: "/latest" }.to_string();
	let data = Url::from((filters, page))
		.get()
		.header(
			"User-Agent",
			"Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
		)
		.header("Referer", &referer)
		.json()?
		.as_object()?
		.get("data")
		.as_object()?;
	let manga = data
		.get("items")
		.as_array()?
		.map(|val| {
			let item = val.as_object()?;
			let id = item.get("manga_code").as_string()?.read();

			let cover = item.get("cover").as_string().unwrap_or_default().read();

			let title = item.get("name").as_string().unwrap_or_default().read();

			let url = Url::Manga { id: &id }.into();

			Ok(Manga {
				id,
				cover,
				title,
				url,
				..Default::default()
			})
		})
		.collect::<Result<_>>()?;

	let has_more = !data.get("isEnd").as_bool()?;

	Ok(MangaPageResult { manga, has_more })
}
