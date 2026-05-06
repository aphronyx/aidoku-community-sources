#![expect(clippy::pub_use, reason = "cleaner")]

mod ranking;
mod search;
mod session;

pub use {ranking::Root as Ranking, search::Root as Search, session::Root as Session};

use {
	crate::net::Url,
	aidoku::{Manga, serde::Deserialize},
};

#[derive(Deserialize)]
struct MangaItem<'a> {
	alias: &'a str,
	title: &'a str,
	thumbnails: [Thumbnail<'a>; 1],
	creators: &'a str,
}

impl MangaItem<'_> {
	fn to_manga(&self) -> Manga {
		let key = self.alias.into();

		let title = self.title.into();

		let cover = self.thumbnails[0].image_path.into();

		let authors = self
			.creators
			.split(',')
			.map(|creator| creator.trim().into())
			.collect();

		let url = Url::Manga { key: self.alias }.to_string().ok();

		Manga {
			key,
			title,
			cover: Some(cover),
			authors: Some(authors),
			url,
			..Default::default()
		}
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Thumbnail<'a> {
	image_path: &'a str,
}
