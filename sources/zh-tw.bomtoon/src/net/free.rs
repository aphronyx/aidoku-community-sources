use aidoku::serde::Serialize;

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum Type {
	Freetime,
	// Free,
}
