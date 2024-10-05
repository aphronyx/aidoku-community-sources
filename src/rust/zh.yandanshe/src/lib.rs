#![no_std]

mod helper;

use aidoku::{
	error::Result,
	prelude::{get_manga_list, handle_notification, initialize},
	std::{String, Vec},
	Filter, Manga, MangaContentRating, MangaPageResult, MangaStatus,
};
use helper::{setting::change_rate_limit, url::Url};

#[initialize]
fn initialize() {
	change_rate_limit();
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let filters_page = Url::from((filters, page)).html()?;
	let manga = filters_page
		.select("article.item")
		.array()
		.map(|val| {
			let item = val.as_node()?;
			let url = item.select("a").attr("href").read();

			let id = url.replace(|c: char| !c.is_ascii_digit(), "");

			let cover = item.select("img").attr("src").read();

			let title = item.select("h3").text().read();

			let categories = if item.select("span.label").text().read() == "會員" {
				["會員專區".into()].into()
			} else {
				[].into()
			};

			let status = if item.select("footer").text().read().starts_with('全') {
				MangaStatus::Completed
			} else {
				MangaStatus::Ongoing
			};

			let nsfw = MangaContentRating::Nsfw;

			Ok(Manga {
				id,
				cover,
				title,
				url,
				categories,
				status,
				nsfw,
				..Default::default()
			})
		})
		.collect::<Result<_>>()?;

	let has_more = !filters_page.select("li.next-page a").array().is_empty();

	Ok(MangaPageResult { manga, has_more })
}

#[expect(clippy::needless_pass_by_value)]
#[handle_notification]
fn handle_notification(notification: String) {
	match notification.as_str() {
		"changeRequests" | "changePeriod" => change_rate_limit(),
		_ => (),
	}
}
