use aidoku::{
	error::{AidokuError, NodeError, Result},
	helpers::uri::{encode_uri_component, QueryParameters},
	prelude::format,
	std::{html::Node, net::Request, String, Vec},
	Filter, FilterType,
};
use alloc::string::ToString as _;
use core::fmt::{Display, Formatter, Result as FmtResult};
use strum::{Display, FromRepr, IntoStaticStr};

#[expect(private_interfaces)]
#[derive(Display)]
#[strum(prefix = "https://yandanshe.com")]
pub enum Url {
	#[strum(to_string = "/{genre}{status}/page/{page}/?{query}")]
	Filters {
		genre: Genre,
		status: Status,
		page: i32,
		query: FiltersQuery,
	},

	#[strum(to_string = "/page/{page}/?{query}")]
	Search { page: i32, query: SearchQuery },

	#[strum(to_string = "/page/{page}/")]
	Home { page: i32 },

	#[strum(to_string = "/{id}/")]
	Manga { id: String },

	#[strum(to_string = "/{manga_id}/{chapter_id}/")]
	Chapter {
		manga_id: String,
		chapter_id: String,
	},
}

impl Url {
	pub const fn home(page: i32) -> Self {
		Self::Home { page }
	}

	pub fn html(&self) -> Result<Node> {
		let html = Request::get(self.to_string()).html()?;

		let is_blocked = html.select("title").text().read() == "您已被臨時封鎖";
		if is_blocked {
			return Err(AidokuError::from(NodeError::ParseError));
		}

		Ok(html)
	}

	pub fn search<S: AsRef<str>>(keyword: S, page: i32) -> Self {
		let query = SearchQuery::new(keyword.as_ref().into());

		Self::Search { page, query }
	}

	pub fn manga<S: AsRef<str>>(id: S) -> Self {
		Self::Manga {
			id: id.as_ref().into(),
		}
	}

	pub fn chapter<M: AsRef<str>, C: AsRef<str>>(manga_id: M, chapter_id: C) -> Self {
		Self::Chapter {
			manga_id: manga_id.as_ref().into(),
			chapter_id: chapter_id.as_ref().into(),
		}
	}

	const fn filters(
		genre: Genre,
		status: Status,
		tags: Vec<String>,
		mode: Mode,
		sort_by: Sort,
		page: i32,
	) -> Self {
		let query = FiltersQuery::new(tags, mode, sort_by);

		Self::Filters {
			genre,
			status,
			page,
			query,
		}
	}
}

impl From<(Vec<Filter>, i32)> for Url {
	fn from((filters, page): (Vec<Filter>, i32)) -> Self {
		macro_rules! init {
			($($filter:ident, $Filter:ident);+) => {
				$(let mut $filter = $Filter::default();)+
			};
		}
		init!(genre, Genre; status, Status; mode, Mode; sort_by, Sort);

		let mut tags = Vec::new();

		for filter in filters {
			#[expect(clippy::wildcard_enum_match_arm)]
			match filter.kind {
				FilterType::Select => {
					macro_rules! get_filter {
						($Filter:ident) => {
							filter
								.value
								.as_int()
								.ok()
								.and_then(|i| {
									let index = i.try_into().ok()?;

									$Filter::from_repr(index)
								})
								.unwrap_or_default()
						};
					}
					match filter.name.as_str() {
						"類型" => genre = get_filter!(Genre),
						"連載情形" => status = get_filter!(Status),
						"模式" => mode = get_filter!(Mode),
						_ => continue,
					}
				}

				FilterType::Sort => {
					sort_by = filter
						.value
						.as_object()
						.ok()
						.and_then(|val| {
							let i = val.get("index").as_int().ok()?.try_into().ok()?;

							Sort::from_repr(i)
						})
						.unwrap_or_default();
				}

				FilterType::Title => {
					let keyword = match filter.value.as_string() {
						Ok(str_ref) => str_ref.read(),
						Err(_) => continue,
					};

					return Self::search(keyword, page);
				}

				FilterType::Genre => {
					let is_not_checked = filter.value.as_int().unwrap_or(-1) != 1;
					if is_not_checked {
						continue;
					}

					tags.push(filter.name);
				}

				_ => continue,
			}
		}

		Self::filters(genre, status, tags, mode, sort_by, page)
	}
}

impl From<Url> for String {
	fn from(url: Url) -> Self {
		url.to_string()
	}
}

#[derive(Display, Default, FromRepr)]
enum Genre {
	#[default]
	#[strum(to_string = "bl")]
	Yaoi,

	#[strum(to_string = "bg")]
	Yuri,
}

#[derive(Display, Default, FromRepr)]
enum Status {
	#[default]
	#[strum(to_string = "wj")]
	Completed,

	#[strum(to_string = "lz")]
	Ongoing,
}

#[derive(IntoStaticStr, Clone, Copy, Default, FromRepr)]
enum Mode {
	#[default]
	#[strum(to_string = "+")]
	And,

	#[strum(to_string = ",")]
	Or,
}

#[derive(IntoStaticStr, Clone, Copy, Default, FromRepr)]
enum Sort {
	#[strum(to_string = "like")]
	Likes,

	#[default]
	#[strum(to_string = "")]
	LastUpdated,
}

struct FiltersQuery {
	tags: Vec<String>,
	mode: Mode,
	sort_by: Sort,
}

impl FiltersQuery {
	const fn new(tags: Vec<String>, mode: Mode, sort_by: Sort) -> Self {
		Self {
			tags,
			mode,
			sort_by,
		}
	}
}

impl Display for FiltersQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut query = QueryParameters::new();

		let tags = (!self.tags.is_empty()).then(|| {
			self.tags
				.iter()
				.map(encode_uri_component)
				.collect::<Vec<_>>()
				.join(self.mode.into())
		});
		query.push_encoded("tag", tags.as_deref());

		let sort_by = (!matches!(self.sort_by, Sort::LastUpdated)).then(|| self.sort_by.into());
		query.push_encoded("sort", sort_by);

		write!(f, "{query}")
	}
}

struct SearchQuery {
	keyword: String,
}

impl SearchQuery {
	const fn new(keyword: String) -> Self {
		Self { keyword }
	}
}

impl Display for SearchQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut query = QueryParameters::new();
		query.push("s", Some(&self.keyword));

		write!(f, "{query}")
	}
}
