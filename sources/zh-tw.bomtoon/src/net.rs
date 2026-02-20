use {
	aidoku::{
		imports::net::{Request, RequestError},
		serde::Serialize,
	},
	strum::{AsRefStr, EnumIs},
};

#[derive(AsRefStr, EnumIs, Serialize)]
#[strum(prefix = "https://www.bomtoon.tw")]
#[serde(untagged)]
pub enum Url {
	#[strum(to_string = "/api/auth/session")]
	Session,
}

impl Url {
	pub fn request(&self) -> Result<Request, RequestError> {
		Request::get(self)
	}
}
