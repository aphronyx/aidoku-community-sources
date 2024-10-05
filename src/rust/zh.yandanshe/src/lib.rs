#![no_std]

extern crate alloc;

mod helper;

use aidoku::{
	error::Result,
	helpers::substring::Substring as _,
	prelude::{
		format, get_chapter_list, get_manga_details, get_manga_list, get_manga_listing,
		handle_notification, initialize,
	},
	std::{String, Vec},
	Chapter, Filter, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer,
};
use alloc::string::ToString as _;
use helper::{setting::change_rate_limit, to_aidoku_error, url::Url, MangaListPage as _};

#[initialize]
fn initialize() {
	change_rate_limit();
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	Url::from((filters, page)).html()?.get_manga_page_result()
}

#[expect(clippy::needless_pass_by_value)]
#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	if listing.name != "首頁" {
		let msg = format!("Listing unimplemented: {}", listing.name);

		return Err(to_aidoku_error(msg));
	}

	Url::home(page).html()?.get_manga_page_result()
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = Url::manga(&id);

	let manga_page = url.html()?;
	let title = manga_page.select("h1").text().read();

	let cover = Url::search(&title, 1)
		.html()
		.map(|search_page| {
			let selector = format!("a[href*=/{id}/] img");

			search_page.select(selector).attr("src").read()
		})
		.unwrap_or_default();

	let author = manga_page.select("span.item-author").text().read();

	let description = manga_page
		.select("blockquote p")
		.array()
		.filter_map(|val| {
			let text = val.as_node().ok()?.text().read();

			Some(text)
		})
		.collect::<Vec<_>>()
		.join("\n\n");

	let mut nsfw = MangaContentRating::Nsfw;
	let mut viewer = MangaViewer::default();
	let categories = manga_page
		.select("a[rel=tag]")
		.array()
		.filter_map(|val| {
			let tag = val.as_node().ok()?.text().read();

			match tag.as_str() {
				"清水向" | "清水" => nsfw = MangaContentRating::Safe,
				"條漫" => viewer = MangaViewer::Scroll,
				_ => (),
			}

			Some(tag)
		})
		.collect();

	let status = match manga_page
		.select("a[rel*=category]")
		.text()
		.read()
		.substring_after_last('·')
	{
		Some("完結") => MangaStatus::Completed,
		Some("連載") => MangaStatus::Ongoing,
		_ => MangaStatus::Unknown,
	};

	Ok(Manga {
		id,
		cover,
		title,
		author,
		description,
		url: url.into(),
		categories,
		status,
		nsfw,
		viewer,
		..Default::default()
	})
}

#[expect(clippy::needless_pass_by_value)]
#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let manga_page = Url::manga(&manga_id).html()?;
	let chapters_len = match manga_page.select(".post-page-numbers").array().len() {
		0 => 1,
		len => len,
	};
	let mut chapters = (1..=chapters_len)
		.map(|n| {
			let id = n.to_string();

			#[expect(clippy::cast_precision_loss, clippy::as_conversions)]
			let chapter = n as _;

			let url = Url::chapter(&manga_id, &id).into();

			let lang = "zh".into();

			Chapter {
				id,
				chapter,
				url,
				lang,
				..Default::default()
			}
		})
		.rev()
		.collect::<Vec<_>>();
	if let Some(last_chapter) = chapters.first_mut() {
		last_chapter.date_updated =
			manga_page
				.select("span.item-time")
				.text()
				.as_date("yyyy-MM-dd", None, None);
	}

	Ok(chapters)
}

#[expect(clippy::needless_pass_by_value)]
#[handle_notification]
fn handle_notification(notification: String) {
	match notification.as_str() {
		"changeRequests" | "changePeriod" => change_rate_limit(),
		_ => (),
	}
}
