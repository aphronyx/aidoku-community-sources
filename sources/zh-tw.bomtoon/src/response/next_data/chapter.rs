use aidoku::{Page, PageContent, alloc::Vec, serde::Deserialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProps<'a> {
	#[serde(borrow)]
	episode_data: EpisodeData<'a>,
}

impl PageProps<'_> {
	pub fn pages(&self) -> Vec<Page> {
		self.episode_data
			.result
			.images
			.iter()
			.map(Image::to_page)
			.collect()
	}
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
