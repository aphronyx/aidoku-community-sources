#![expect(clippy::pub_use, reason = "cleaner")]

mod manga;
mod ranking;
mod search;
mod session;

pub use {
	manga::{Root as Manga, UpdateManga},
	ranking::Root as Ranking,
	search::Root as Search,
	session::{AccessToken, Root as Session},
};

use {
	crate::net::{ThumbnailType, Url},
	aidoku::{
		Viewer,
		alloc::{String, Vec},
		serde::Deserialize,
	},
};

#[derive(Deserialize)]
struct MangaItem<'a> {
	alias: &'a str,
	title: &'a str,
	thumbnails: [Thumbnail<'a>; 1],
	creators: &'a str,
}

impl MangaItem<'_> {
	fn to_manga(&self) -> aidoku::Manga {
		let key = self.alias.into();

		let title = self.title.into();

		let cover = self.thumbnails[0].image_path.into();

		let authors = self
			.creators
			.split(',')
			.map(|creator| creator.trim().into())
			.collect();

		let url = Url::MangaPage { key: self.alias }.to_string().ok();

		aidoku::Manga {
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
	r#type: ThumbnailType,
}

trait Tags {
	fn get_viewer(&self) -> Viewer;
}

impl Tags for Vec<String> {
	fn get_viewer(&self) -> Viewer {
		if self.iter().any(|tag| tag == "翻頁式漫畫") {
			Viewer::Unknown
		} else {
			Viewer::Webtoon
		}
	}
}
