use {
	crate::net::Url,
	aidoku::{
		ContentRating, Manga, MangaPageResult, MangaStatus, Viewer,
		alloc::{Vec, borrow::Cow},
		serde::Deserialize,
	},
	arrayvec::ArrayString,
};

#[derive(Deserialize)]
pub struct Root<'a> {
	#[serde(borrow)]
	data: Data<'a>,
}

impl Root<'_> {
	pub const fn next_pagination(&self) -> ArrayString<31> {
		self.data.next_pagination
	}

	pub fn manga_page_result(&self) -> MangaPageResult {
		self.data.to_manga_page_result()
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
	#[serde(borrow)]
	contents: Vec<Content<'a>>,
	has_next: bool,
	next_pagination: ArrayString<31>,
}

impl Data<'_> {
	fn to_manga_page_result(&self) -> MangaPageResult {
		let entries = self.contents.iter().map(Content::to_manga).collect();

		let has_next_page = self.has_next;

		MangaPageResult {
			entries,
			has_next_page,
		}
	}
}

#[derive(Deserialize)]
struct Content<'a> {
	alias: &'a str,
	title: &'a str,
	thumbnails: [Thumbnail<'a>; 1],
	tag: Cow<'a, str>,
	creators: &'a str,
}

impl Content<'_> {
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

		let tags = self.tag.split(',').map(Into::into).collect::<Vec<_>>();
		let status = if tags.iter().any(|tag| tag == "完結") {
			MangaStatus::Completed
		} else {
			MangaStatus::Ongoing
		};

		let content_rating = if tags.iter().any(|tag| tag == "清水") {
			ContentRating::Safe
		} else {
			ContentRating::NSFW
		};

		let viewer = if tags.iter().any(|tag| tag == "翻頁式漫畫") {
			Viewer::Unknown
		} else {
			Viewer::Webtoon
		};

		Manga {
			key,
			title,
			cover: Some(cover),
			authors: Some(authors),
			url,
			tags: Some(tags),
			status,
			content_rating,
			viewer,
			..Default::default()
		}
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Thumbnail<'a> {
	image_path: &'a str,
}
