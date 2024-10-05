use aidoku::std::{
	defaults::defaults_get,
	net::{set_rate_limit, set_rate_limit_period},
};

pub fn change_rate_limit() {
	macro_rules! get_setting {
		($key:expr ,$default:expr) => {
			defaults_get($key)
				.ok()
				.and_then(|val| val.as_int().ok()?.try_into().ok())
				.unwrap_or($default)
		};
	}
	let requests = get_setting!("requests", 5);
	set_rate_limit(requests);

	let period = get_setting!("period", 5);
	set_rate_limit_period(period);
}
