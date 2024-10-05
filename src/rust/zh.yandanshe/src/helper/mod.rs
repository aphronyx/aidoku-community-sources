pub mod image;
pub mod setting;
pub mod url;

use aidoku::{
	error::{AidokuError, AidokuErrorKind, Result},
	prelude::println,
	std::html::Node,
	Manga, MangaContentRating, MangaPageResult, MangaStatus,
};
use core::fmt::Display;

pub trait MangaListPage {
	fn get_manga_page_result(self) -> Result<MangaPageResult>;
}

impl MangaListPage for Node {
	fn get_manga_page_result(self) -> Result<MangaPageResult> {
		let manga = self
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

		let has_more = !self.select("li.next-page a").array().is_empty();

		Ok(MangaPageResult { manga, has_more })
	}
}

pub fn to_aidoku_error<E: Display>(msg: E) -> AidokuError {
	println!("{msg}");

	let reason = AidokuErrorKind::Unimplemented;

	AidokuError { reason }
}
