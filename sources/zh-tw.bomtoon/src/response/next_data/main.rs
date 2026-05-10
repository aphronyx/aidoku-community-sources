use {
	super::super::MangaItem,
	crate::net::{Url, free},
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
	pub fn home_components(&self) -> [HomeComponent; 3] {
		[
			self.main.get_banners_home_component(),
			self.main.get_quick_menu_home_component(),
			self.main.get_newest_home_component(),
		]
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum UrlTarget {
	Event,
	EventUrl,
	FreetimeComic,
	Gift,
	Shop,
	ShortComic,
	Url,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Main<'a> {
	#[serde(borrow)]
	banners: Vec<Banner<'a>>,
	quick_menu: [QuickMenuItem<'a>; 8],
	newest: Vec<MangaItem<'a>>,
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

	fn get_quick_menu_home_component(&self) -> HomeComponent {
		let links = self.quick_menu.iter().map(QuickMenuItem::to_link).collect();
		let value = HomeComponentValue::Links(links);
		HomeComponent {
			value,
			..Default::default()
		}
	}

	fn get_newest_home_component(&self) -> HomeComponent {
		let links = self.newest.iter().filter_map(MangaItem::to_link).collect();
		let value = HomeComponentValue::ImageScroller {
			links,
			auto_scroll_interval: None,
			width: None,
			height: Some(224),
		};
		HomeComponent {
			title: Some("熱騰騰人氣新作!".into()),
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuickMenuItem<'a> {
	title: &'a str,
	url_target: UrlTarget,
	url: &'a str,
}

impl QuickMenuItem<'_> {
	fn to_link(&self) -> Link {
		let title = self.title.into();

		let value = match self.url_target {
			UrlTarget::Event => Url::EventPage.to_string().map(LinkValue::Url).ok(),
			UrlTarget::EventUrl => Url::Event(self.url).to_string().map(LinkValue::Url).ok(),
			UrlTarget::FreetimeComic => Url::Free {
				f: Some(free::Type::Freetime),
			}
			.to_string()
			.map(LinkValue::Url)
			.ok(),
			UrlTarget::Gift => Url::Gift.to_string().map(LinkValue::Url).ok(),
			UrlTarget::Shop => Url::Shop.to_string().map(LinkValue::Url).ok(),
			UrlTarget::ShortComic => Url::Short.to_string().map(LinkValue::Url).ok(),
			UrlTarget::Url => Some(LinkValue::Url(self.url.into())),
		};

		Link {
			title,
			value,
			..Default::default()
		}
	}
}
