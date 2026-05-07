use {
	super::{MangaItem, Tags as _},
	aidoku::{
		ContentRating, Manga, MangaPageResult, MangaStatus,
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
	#[serde(flatten, borrow)]
	manga_item: MangaItem<'a>,
	tag: Cow<'a, str>,
}

impl Content<'_> {
	fn to_manga(&self) -> Manga {
		let mut manga = self.manga_item.to_manga();

		let tags = self.tag.split(',').map(Into::into).collect::<Vec<_>>();
		manga.status = if tags.iter().any(|tag| tag == "完結") {
			MangaStatus::Completed
		} else {
			MangaStatus::Ongoing
		};

		manga.content_rating = if tags.iter().any(|tag| tag == "清水") {
			ContentRating::Safe
		} else {
			ContentRating::NSFW
		};

		manga.viewer = tags.get_viewer();

		manga.tags = Some(tags);

		manga
	}
}
