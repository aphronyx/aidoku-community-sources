use {aidoku::serde::Serialize, strum::FromRepr};

#[derive(Serialize, Default, FromRepr)]
#[serde(rename_all = "UPPERCASE")]
#[repr(i32)]
pub enum Type {
	#[default]
	Realtime,
	Weekly,
	Monthly,
	Total,
}
