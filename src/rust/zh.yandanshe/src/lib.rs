#![no_std]

mod helper;

use aidoku::{
	prelude::{handle_notification, initialize},
	std::String,
};
use helper::setting::change_rate_limit;

#[initialize]
fn initialize() {
	change_rate_limit();
}

#[expect(clippy::needless_pass_by_value)]
#[handle_notification]
fn handle_notification(notification: String) {
	match notification.as_str() {
		"changeRequests" | "changePeriod" => change_rate_limit(),
		_ => (),
	}
}
