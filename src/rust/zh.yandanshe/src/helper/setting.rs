use aidoku::std::{defaults::defaults_get, net::set_rate_limit};

pub fn change_rate_limit() {
	let requests = defaults_get("requests")
		.ok()
		.and_then(|val| val.as_int().ok()?.try_into().ok())
		.unwrap_or(5);
	set_rate_limit(requests);
}
