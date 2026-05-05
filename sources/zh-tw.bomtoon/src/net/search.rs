use {
	aidoku::{
		alloc::{Vec, collections::BTreeSet},
		serde::{Serialize, Serializer},
	},
	strum::{AsRefStr, Display},
};

#[derive(Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Type {
	All,
	Tag,
}

#[derive(AsRefStr, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ThumbnailType {
	Detail,
	// DetailNonAdult,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
	Input,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortBy {
	Popular,
	// Latest,
}

pub fn thumbnail_types<S: Serializer>(
	types: &BTreeSet<ThumbnailType>,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	types
		.iter()
		.map(AsRef::as_ref)
		.collect::<Vec<_>>()
		.join(",")
		.serialize(serializer)
}
