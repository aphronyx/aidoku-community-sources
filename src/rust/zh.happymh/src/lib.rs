#![no_std]

extern crate alloc;

mod helper;

use aidoku::{
	error::Result,
	prelude::{get_manga_details, get_manga_list},
	std::{String, Vec},
	Filter, Manga, MangaPageResult, MangaStatus, MangaViewer,
};
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

#[expect(clippy::needless_pass_by_value)]
#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = Url::Manga { id: &id };

	let manga_page = url.get().html()?;
	let cover = manga_page.select("div.mg-cover mip-img").attr("src").read();

	let title = manga_page.select("h2.mg-title").text().read();

	let author = manga_page
		.select("p.mg-sub-title a")
		.array()
		.filter_map(|val| {
			let author = val.as_node().ok()?.text().read();

			Some(author)
		})
		.collect::<Vec<_>>()
		.join("、");

	let description = manga_page
		.select("div.manga-introduction mip-showmore")
		.text()
		.read();

	let mut viewer = MangaViewer::default();
	let categories = manga_page
		.select("p.mg-cate a")
		.array()
		.filter_map(|val| {
			let genre = val.as_node().ok()?.text().read();

			if genre == "长条" {
				viewer = MangaViewer::Scroll;
			}

			Some(genre)
		})
		.collect();

	let status = match manga_page
		.select("div.ongoing-status")
		.text()
		.read()
		.as_str()
	{
		"连载中" => MangaStatus::Ongoing,
		"已完结" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};

	Ok(Manga {
		id: id.clone(),
		cover,
		title,
		author,
		description,
		url: url.into(),
		categories,
		status,
		viewer,
		..Default::default()
	})
}
