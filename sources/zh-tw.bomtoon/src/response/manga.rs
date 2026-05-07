use {
	super::{Tags as _, Thumbnail},
	crate::net::Url,
	aidoku::{
		Chapter, ContentRating, Manga,
		alloc::{
			String, Vec,
			borrow::{Cow, ToOwned as _},
			format,
			string::ToString as _,
		},
		helpers::string::PlainText as _,
		imports::{defaults::defaults_get, std::current_date},
		serde::Deserialize,
	},
	chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese as _},
	strum::EnumIs,
};

#[derive(Deserialize)]
pub struct Root<'a> {
	#[serde(borrow)]
	data: Data<'a>,
}

impl Root<'_> {
	pub fn covers(&self) -> Vec<String> {
		self.data
			.thumbnails
			.iter()
			.map(|thumbnail| thumbnail.image_path.into())
			.collect()
	}
}

pub trait UpdateManga {
	fn update_details(&mut self, updated_manga: &Root);
	fn update_chapters(&mut self, updated_manga: &Root);
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

	fn update_chapters(&mut self, updated_manga: &Root) {
		let hides_preview = defaults_get::<bool>("hidesPreview").unwrap_or_default();
		let chapters = updated_manga
			.data
			.episodes
			.iter()
			.filter(|episode| !hides_preview || episode.alias != "f1")
			.map(|episode| episode.to_chapter(updated_manga.data.alias))
			.rev()
			.collect();
		self.chapters = Some(chapters);
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

#[derive(Deserialize, EnumIs)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ChapterThumbnailType {
	Common,
	NonAdult,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
enum PurchaseStatus {
	Possession,
	Rent,
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
	episodes: Vec<Episode<'a>>,
	thumbnails: Vec<Thumbnail<'a>>,
	creators: Vec<Creator<'a>>,
	is_mixed: bool,
}

#[derive(Deserialize)]
struct Tag<'a> {
	name: Cow<'a, str>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Episode<'a> {
	alias: &'a str,
	title: &'a str,
	sub_title: Option<&'a str>,
	opened_at: u64,
	is_login: bool,
	possession_coin: u8,
	thumbnails: Vec<ChapterThumbnail<'a>>,
	purchase_status: Option<PurchaseStatus>,
	expired_at: u64,
	is_rent_gift: bool,
	is_rent_freetime: bool,
}

impl Episode<'_> {
	fn to_chapter(&self, manga_key: &str) -> Chapter {
		let key = self.alias.into();

		let mut chapter_title = self.title.to_owned();
		let (volume_number, chapter_number) = self
			.alias
			.parse::<f32>()
			.map(|n| {
				if chapter_title.contains('冊') {
					if let Ok(n_in_chinese) = n.to_chinese(
						ChineseVariant::Traditional,
						ChineseCase::Lower,
						ChineseCountMethod::TenThousand,
					) && let Some(title) = chapter_title
						.strip_prefix('第')
						.and_then(|title| title.strip_prefix(&n_in_chinese)?.strip_prefix('冊'))
					{
						chapter_title = title.into();
					}
					return (Some(n), None);
				}

				if let Some(title) = chapter_title
					.strip_prefix('第')
					.and_then(|title| title.strip_prefix(&n.to_string())?.strip_prefix('話'))
				{
					chapter_title = title.into();
				}
				(None, Some(n))
			})
			.unwrap_or_default();

		let title = match (
			chapter_title.is_empty(),
			self.sub_title.filter(|subtitle| !subtitle.is_empty()),
		) {
			(true, None) => None,
			(true, Some(subtitle)) => Some(subtitle.into()),
			(false, None) => Some(chapter_title),
			(false, Some(subtitle)) => Some(format!("{chapter_title}：{subtitle}")),
		};

		#[expect(clippy::integer_division, reason = "should be an integer in seconds")]
		let date_uploaded = (self.opened_at / 1_000).try_into().ok();

		let url = Url::ChapterPage {
			manga_key,
			key: self.alias,
		}
		.to_string()
		.ok();

		let thumbnail = self.thumbnails.iter().find_map(|thumbnail| {
			thumbnail
				.r#type
				.is_common()
				.then(|| thumbnail.image_path.into())
		});

		let locked = self.is_login
			&& (defaults_get::<String>("login").is_none()
				|| !(self.possession_coin == 0
					|| self.purchase_status == Some(PurchaseStatus::Possession)
					|| (self.purchase_status == Some(PurchaseStatus::Rent)
						&& u64::try_from(current_date())
							.ok()
							.and_then(|seconds| seconds.checked_mul(1_000))
							.unwrap_or(0) < self.expired_at)
					|| self.is_rent_gift
					|| self.is_rent_freetime));

		Chapter {
			key,
			title,
			chapter_number,
			volume_number,
			date_uploaded,
			url,
			thumbnail,
			locked,
			..Default::default()
		}
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChapterThumbnail<'a> {
	image_path: &'a str,
	r#type: ChapterThumbnailType,
}

#[derive(Deserialize)]
struct Creator<'a> {
	name: &'a str,
	// HACK(artists): currently not supported
	// r#type: CreatorType,
}
