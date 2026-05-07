use {aidoku::serde::Serialize, strum::Display};

#[derive(Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Type {
	All,
	Tag,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
	Input,
	Click,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortBy {
	Popular,
	// Latest,
}
