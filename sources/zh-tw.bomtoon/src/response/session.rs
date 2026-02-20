use {
	aidoku::{
		imports::defaults::defaults_set_data,
		serde::{Deserialize, Serialize},
	},
	arrayvec::ArrayString,
};

#[derive(Deserialize)]
pub struct Root {
	user: User,
}

impl Root {
	pub fn refresh_access_token(&self) {
		defaults_set_data("accessToken", self.user.access_token);
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
	access_token: AccessToken,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct AccessToken {
	token: ArrayString<528>,
	expired_at: u64,
}
