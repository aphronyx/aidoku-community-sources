use aidoku::{
	Page, PageContent,
	alloc::Vec,
	imports::net::{Request, RequestError},
	serde::Deserialize,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProps<'a> {
	#[serde(borrow)]
	episode_data: EpisodeData<'a>,
}

impl PageProps<'_> {
	pub fn pages(&self) -> Result<Vec<Page>, RequestError> {
		let requests = self
			.episode_data
			.result
			.images
			.iter()
			.map(|image| Request::get(image.image_path))
			.collect::<Result<Vec<_>, _>>()?;
		Request::send_all(requests)
			.into_iter()
			.map(|response| {
				let image = response?.get_image()?;
				Ok(Page {
					content: PageContent::image(image),
					..Default::default()
				})
			})
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
