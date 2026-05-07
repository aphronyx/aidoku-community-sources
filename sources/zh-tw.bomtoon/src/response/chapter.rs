use aidoku::{Page, PageContent, alloc::Vec, serde::Deserialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root<'a> {
	#[serde(borrow)]
	page_props: PageProps<'a>,
}

impl Root<'_> {
	pub fn pages(&self) -> Vec<Page> {
		self.page_props
			.episode_data
			.result
			.images
			.iter()
			.map(Image::to_page)
			.collect()
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageProps<'a> {
	#[serde(borrow)]
	episode_data: EpisodeData<'a>,
}

#[derive(Deserialize)]
struct EpisodeData<'a> {
	#[serde(borrow)]
	result: EpisodeResult<'a>,
}

#[derive(Deserialize)]
struct EpisodeResult<'a> {
	#[serde(borrow)]
	images: Vec<Image<'a>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image<'a> {
	image_path: &'a str,
}

impl Image<'_> {
	fn to_page(&self) -> Page {
		let content = PageContent::url(self.image_path);
		Page {
			content,
			..Default::default()
		}
	}
}
