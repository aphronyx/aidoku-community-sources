#![no_std]

mod helper;

use aidoku::{
	error::Result,
	prelude::{format, get_manga_list, get_manga_listing, handle_notification, initialize},
	std::{String, Vec},
	Filter, Listing, MangaPageResult,
};
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

#[expect(clippy::needless_pass_by_value)]
#[handle_notification]
fn handle_notification(notification: String) {
	match notification.as_str() {
		"changeRequests" | "changePeriod" => change_rate_limit(),
		_ => (),
	}
}
