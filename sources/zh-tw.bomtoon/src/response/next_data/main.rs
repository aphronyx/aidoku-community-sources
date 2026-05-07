use {
	crate::net::Url,
	aidoku::{
		HomeComponent, HomeComponentValue, Link, LinkValue, Manga, alloc::Vec, serde::Deserialize,
	},
};

#[derive(Deserialize)]
pub struct PageProps<'a> {
	#[serde(borrow)]
	main: Main<'a>,
}

impl PageProps<'_> {
	pub fn home_components(&self) -> [HomeComponent; 1] {
		[self.main.get_banners_home_component()]
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Target {
	Contents,
	Event,
	Main,
	Payment,
	Play,
	Pick,
	Url,
}

#[derive(Deserialize)]
struct Main<'a> {
	#[serde(borrow)]
	banners: Vec<Banner<'a>>,
}

impl Main<'_> {
	fn get_banners_home_component(&self) -> HomeComponent {
		let links = self
			.banners
			.iter()
			.filter_map(|banner| banner.banner_detail_info[0].to_link())
			.collect();
		let value = HomeComponentValue::ImageScroller {
			links,
			auto_scroll_interval: Some(10.0 / 3.0),
			width: None,
			height: Some(330),
		};
		HomeComponent {
			value,
			..Default::default()
		}
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Banner<'a> {
	#[serde(borrow)]
	banner_detail_info: [BannerDetailInfo<'a>; 1],
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BannerDetailInfo<'a> {
	link_info: LinkInfo<'a>,
	#[serde(borrow)]
	thumbnails: [Thumbnail<'a>; 10],
}

impl BannerDetailInfo<'_> {
	fn to_link(&self) -> Option<Link> {
		let image_url = self
			.thumbnails
			.iter()
			.find(|thumbnail| !thumbnail.image_path.is_empty() && thumbnail.is_adult)?
			.image_path
			.into();

		let value = self.link_info.to_link_value();

		Some(Link {
			image_url: Some(image_url),
			value,
			..Default::default()
		})
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkInfo<'a> {
	params: &'a str,
	target: Target,
}

impl LinkInfo<'_> {
	fn to_link_value(&self) -> Option<LinkValue> {
		let value = match self.target {
			Target::Contents => LinkValue::Manga(Manga {
				key: self.params.into(),
				..Default::default()
			}),
			Target::Event => LinkValue::Url(Url::Event(self.params).to_string().ok()?),
			Target::Main => return None,
			Target::Payment => LinkValue::Url(Url::Shop.to_string().ok()?),
			Target::Play => LinkValue::Url(Url::Play.to_string().ok()?),
			Target::Pick => LinkValue::Url(Url::Pick.to_string().ok()?),
			Target::Url => LinkValue::Url(self.params.into()),
		};
		Some(value)
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Thumbnail<'a> {
	image_path: &'a str,
	is_adult: bool,
}
