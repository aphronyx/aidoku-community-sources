use {
	super::MangaItem,
	aidoku::{MangaPageResult, alloc::Vec},
	serde::Deserialize,
};

#[derive(Deserialize)]
pub struct Root<'a> {
	#[serde(borrow)]
	data: Vec<MangaItem<'a>>,
}

impl Root<'_> {
	pub fn manga_page_result(&self) -> MangaPageResult {
		let entries = self.data.iter().map(MangaItem::to_manga).collect();

		MangaPageResult {
			entries,
			has_next_page: false,
		}
	}
}
