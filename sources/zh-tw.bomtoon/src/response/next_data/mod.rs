mod chapter;
mod main;

use {
	aidoku::{
		HomeLayout, Page,
		alloc::Vec,
		bail, error,
		imports::{defaults::defaults_set_data, html::Document},
		serde::Deserialize,
	},
	arrayvec::ArrayString,
};

pub trait NextData {
	fn next_data(&self) -> aidoku::Result<Root>;
}

impl NextData for Document {
	fn next_data(&self) -> aidoku::Result<Root> {
		let json = self
			.select_first("script#__NEXT_DATA__")
			.ok_or_else(|| error!("no element found for selector: `script#__NEXT_DATA__`"))?
			.data()
			.ok_or_else(|| error!("no script content"))?;
		let next_data = serde_json::from_str::<Root>(&json)?;
		Ok(next_data)
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
	build_id: ArrayString<21>,
}

impl Root {
	pub const fn build_id(&self) -> ArrayString<21> {
		self.build_id
	}

	pub fn update_build_id(&self) {
		defaults_set_data("buildId", self.build_id);
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Props<'a> {
	#[serde(borrow)]
	page_props: PageProps<'a>,
}

impl Props<'_> {
	pub fn pages(&self) -> aidoku::Result<Vec<Page>> {
		if let PageProps::Chapter(ref page_props) = self.page_props {
			let pages = page_props.pages()?;
			Ok(pages)
		} else {
			bail!("not chapter response")
		}
	}

	pub fn home_layout(&self) -> aidoku::Result<HomeLayout> {
		if let PageProps::Main(ref page_props) = self.page_props {
			let components = page_props.home_components().into();
			Ok(HomeLayout { components })
		} else {
			bail!("not main response")
		}
	}
}

#[derive(Deserialize)]
#[serde(untagged)]
enum PageProps<'a> {
	Main(main::PageProps<'a>),
	#[serde(borrow)]
	Chapter(chapter::PageProps<'a>),
}
