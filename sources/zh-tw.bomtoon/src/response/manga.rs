use {
	super::{Tags as _, Thumbnail},
	crate::net::Url,
	aidoku::{
		ContentRating, Manga,
		alloc::{Vec, borrow::Cow, string::ToString as _},
		helpers::string::PlainText as _,
		serde::Deserialize,
	},
};

#[derive(Deserialize)]
pub struct Root<'a> {
	#[serde(borrow)]
	data: Data<'a>,
}

pub trait UpdateManga {
	fn update_details(&mut self, updated_manga: &Root);
}

impl UpdateManga for Manga {
	fn update_details(&mut self, updated_manga: &Root) {
		self.key = updated_manga.data.alias.into();

		self.title = updated_manga.data.title.into();

		self.cover = updated_manga.data.thumbnails.iter().find_map(|thumbnail| {
			thumbnail
				.r#type
				.is_vertical()
				.then(|| thumbnail.image_path.into())
		});

		// HACK(artists): currently not supported
		// let mut artists = Vec::new();
		// let mut authors = Vec::new();
		// for creator in &updated_manga.data.creators {
		// 	let name = creator.name.into();
		// 	if creator.r#type.is_artist() {
		// 		artists.push(name);
		// 	} else {
		// 		authors.push(name);
		// 	}
		// }
		// self.artists = Some(artists);
		let authors = updated_manga
			.data
			.creators
			.iter()
			.map(|creator| creator.name.into())
			.collect();
		self.authors = Some(authors);

		let description = updated_manga.data.synopsis.escape_markdown();
		self.description = Some(description);

		self.url = Url::MangaPage {
			key: updated_manga.data.alias,
		}
		.to_string()
		.ok();

		let tags = updated_manga
			.data
			.tags
			.iter()
			.map(|tag| tag.name.to_string())
			.collect::<Vec<_>>();
		self.viewer = tags.get_viewer();

		self.tags = Some(tags);

		self.status = updated_manga.data.status;

		self.content_rating = if updated_manga.data.is_mixed {
			ContentRating::NSFW
		} else {
			ContentRating::Safe
		};
	}
}

#[derive(Deserialize)]
#[serde(remote = "aidoku::MangaStatus")]
enum MangaStatus {
	#[serde(rename = "SCHEDULED")]
	Ongoing,
	#[serde(rename = "COMPLETED")]
	Completed,
	#[serde(rename = "PAUSED")]
	Hiatus,
}

// HACK(artists): currently not supported
// #[derive(Deserialize, EnumIs)]
// #[serde(rename_all = "UPPERCASE")]
// enum CreatorType {
// 	Author,
// 	Original,
// 	Artist,
// }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
	alias: &'a str,
	title: &'a str,
	synopsis: Cow<'a, str>,
	#[serde(with = "MangaStatus")]
	status: aidoku::MangaStatus,
	tags: Vec<Tag<'a>>,
	thumbnails: Vec<Thumbnail<'a>>,
	creators: Vec<Creator<'a>>,
	is_mixed: bool,
}

#[derive(Deserialize)]
struct Tag<'a> {
	name: Cow<'a, str>,
}

#[derive(Deserialize)]
struct Creator<'a> {
	name: &'a str,
	// HACK(artists): currently not supported
	// r#type: CreatorType,
}
