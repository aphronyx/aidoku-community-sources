use aidoku::{
	helpers::uri::QueryParameters,
	prelude::format,
	std::{net::Request, String, Vec},
	Filter, FilterType,
};
use alloc::string::ToString as _;
use core::fmt::{Display, Formatter, Result as FmtResult};
use strum_macros::{Display, FromRepr, IntoStaticStr};

#[expect(private_interfaces)]
#[derive(Display)]
#[strum(prefix = "https://m.happymh.com")]
pub enum Url<'a> {
	#[strum(to_string = "/apis/c/index?{query}")]
	Filters { query: FiltersQuery },

	#[strum(to_string = "{path}")]
	Abs { path: &'a str },

	#[strum(to_string = "/manga/{id}")]
	Manga { id: &'a str },
}

impl Url<'_> {
	pub fn get(&self) -> Request {
		Request::get(self.to_string())
	}
}

impl From<Url<'_>> for String {
	fn from(url: Url) -> Self {
		url.to_string()
	}
}

impl From<(Vec<Filter>, i32)> for Url<'_> {
	fn from((filters, page): (Vec<Filter>, i32)) -> Self {
		macro_rules! init {
			($($filter:ident, $Filter:ident);+) => {
				$(let mut $filter = $Filter::default();)+
			};
		}
		init!(
			genre, Genre;
			target_audience, TargetAudience;
			region, Region;
			status, Status;
			sort_by, Sort
		);

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
								.and_then(|val| {
									let i = val.try_into().ok()?;

									$Filter::from_repr(i)
								})
								.unwrap_or_default()
						};
					}
					match filter.name.as_str() {
						"類型" => genre = get_filter!(Genre),
						"受眾" => target_audience = get_filter!(TargetAudience),
						"區域" => region = get_filter!(Region),
						"連載情形" => status = get_filter!(Status),
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

				_ => continue,
			}
		}

		let query = FiltersQuery {
			genre,
			target_audience,
			region,
			status,
			sort_by,
			page,
		};

		Self::Filters { query }
	}
}

#[derive(Default, IntoStaticStr, Clone, Copy, FromRepr)]
#[strum(serialize_all = "lowercase")]
enum Genre {
	#[default]
	#[strum(to_string = "")]
	All,

	Rexue,

	Gedou,

	Wuxia,

	Mohuan,

	Mofa,

	Maoxian,

	Aiqing,

	Gaoxiao,

	Xiaoyuan,

	Kehuan,

	Hougong,

	Lizhi,

	Zhichang,

	Meishi,

	Shehui,

	Heidao,

	Zhanzheng,

	Lishi,

	Xuanyi,

	Jingji,

	Tiyu,

	Kongbu,

	Tuili,

	Shenghuo,

	Weiniang,

	Zhiyu,

	Shengui,

	Sige,

	Baihe,

	Danmei,

	Wudao,

	Zhentan,

	Zhainan,

	Yinyue,

	Mengxi,

	Gufeng,

	Lianai,

	Dushi,

	Xingzhuan,

	Chuanyue,

	Youxi,

	Qita,

	Aiqi,

	Richang,

	Fuhei,

	Guzhuang,

	Xianxia,

	Shenghua,

	Xiuxian,

	Qinggan,

	Gaibian,

	Chunai,

	Weimei,

	Qiangwei,

	Mingxing,

	Lieqi,

	Qingchun,

	Huanxiang,

	Jingqi,

	Caihong,

	Qiwen,

	Quanmou,

	Zhaidou,

	Xianzhiji,

	Zhuangbi,

	Langman,

	Ouxiang,

	Danvzhu,

	Fuchou,

	Nuexin,

	Egao,

	Lingyi,

	Jingxian,

	Chongai,

	Nixi,

	Yaoguai,

	Aimei,

	Tongren,

	Jiakong,

	Zhenren,

	Dongzuo,

	Juwei,

	Gongdou,

	Naodong,

	Mangai,

	Zhandou,

	Sangshi,

	Meishaonv,

	Guaiwu,

	Xitong,

	Zhidou,

	Jijia,

	Gaotian,

	Jiangshi,

	Dianjing,

	Shenmo,

	Yineng,

	Mori,

	Yinv,

	Haokuai,

	Qihuan,

	Shenshi,

	Zhengnengliang,

	Gongting,

	Qinqing,

	Yangcheng,

	Juqing,

	Hanman,

	Qingxiaoshuo,

	Anhei,

	Changtiao,

	Xuanhuan,

	Bazong,

	Ouhuang,

	Shengcun,

	Mengchong,

	Yishijie,

	#[strum(to_string = "C99")]
	C99,

	Jiecao,

	#[strum(to_string = "AA")]
	Aa,

	Yingshihua,

	Oufeng,

	Nvshen,

	Shuanggan,

	Zhuansheng,

	Chengzhang,

	Yixing,

	Xuezu,

	Tuanchong,

	Fantaolu,

	Shuangnanzhu,

	Wudiliu,

	Xinli,

	Yanyi,

	Xingzhuanhuan,

	Zhanren,

	Yangguang,

	Majia,
}

#[derive(Default, IntoStaticStr, Clone, Copy, FromRepr)]
enum TargetAudience {
	#[default]
	#[strum(to_string = "")]
	All,

	#[strum(to_string = "shaonian")]
	Shōnen,

	#[strum(to_string = "shaonv")]
	Shōjo,

	#[strum(to_string = "qingnian")]
	Seinen,

	#[strum(to_string = "BL")]
	Yaoi,

	#[strum(to_string = "GL")]
	Yuri,
}

#[derive(Default, IntoStaticStr, Clone, Copy, FromRepr)]
enum Region {
	#[default]
	#[strum(to_string = "")]
	All,

	#[strum(to_string = "china")]
	Chinese,

	#[strum(to_string = "japan")]
	Japanese,

	#[strum(to_string = "hongkong")]
	TaiwaneseAndHongKong,

	#[strum(to_string = "europe")]
	Western,

	#[strum(to_string = "korea")]
	Korean,

	#[strum(to_string = "other")]
	Others,
}

#[derive(Default, IntoStaticStr, Clone, Copy, FromRepr)]
enum Status {
	#[default]
	#[strum(to_string = "-1")]
	All,

	#[strum(to_string = "0")]
	Ongoing,

	#[strum(to_string = "1")]
	Completed,
}

#[derive(Default, IntoStaticStr, Clone, Copy, FromRepr)]
enum Sort {
	#[default]
	#[strum(to_string = "last_date")]
	LastUpdated,

	#[strum(to_string = "views")]
	Views,
}

struct FiltersQuery {
	genre: Genre,
	target_audience: TargetAudience,
	region: Region,
	status: Status,
	sort_by: Sort,
	page: i32,
}

impl Display for FiltersQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut query = QueryParameters::new();

		macro_rules! assign_filter {
			($filter:ident, $Filter:ident::$Variant:ident) => {
				let $filter = match self.$filter {
					$Filter::$Variant => None,
					rest => Some(rest.into()),
				};
			};

			($($filter:ident, $Filter:ident);+) => {
				$(assign_filter!($filter, $Filter::All);)+
			};
		}

		assign_filter!(
			genre, Genre;
			target_audience, TargetAudience;
			region, Region;
			status, Status
		);
		query.push_encoded("genre", genre);
		query.push_encoded("audience", target_audience);
		query.push_encoded("area", region);
		query.push_encoded("series_status", status);

		assign_filter!(sort_by, Sort::LastUpdated);
		query.push_encoded("order", sort_by);

		let page = self.page.to_string();
		query.push_encoded("pn", Some(&page));

		write!(f, "{query}")
	}
}
