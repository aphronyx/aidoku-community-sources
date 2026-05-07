use {
	crate::net::Url,
	aidoku::{
		imports::{defaults::defaults_set_data, std::current_date},
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

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
	token: ArrayString<528>,
	expired_at: u64,
}

impl AccessToken {
	pub fn token(&self) -> aidoku::Result<ArrayString<528>> {
		let now = current_date()
			.try_into()
			.unwrap_or(u64::MAX)
			.saturating_mul(1_000);
		if now < self.expired_at {
			return Ok(self.token);
		}

		Url::Base.request()?.send()?;
		let session = Url::Session.request()?.json_owned::<Root>()?;
		session.refresh_access_token();
		Ok(session.user.access_token.token)
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
	access_token: AccessToken,
}
