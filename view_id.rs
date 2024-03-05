use error_stack::{Report, ResultExt};
use rand::random;
use rutils::errors;
use schemars::JsonSchema;
use serde::{de::Visitor, Deserialize, Serialize};
use std::{
	fmt::{self, Display},
	str::FromStr,
};

errors!(FailedToDecodeHex, InvalidLength);

#[derive(Debug, Hash, PartialEq, Eq, Clone, JsonSchema)]
pub struct ViewId(#[schemars(with = "String")] [u8; 8]);

impl ViewId {
	pub fn generate() -> ViewId {
		ViewId(random())
	}
}

impl Display for ViewId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&hex::encode(self.0))
	}
}

impl FromStr for ViewId {
	type Err = Report<Error>;

	fn from_str(value: &str) -> Result<ViewId> {
		let bytes = hex::decode(value).change_context(Error::FailedToDecodeHex)?;
		if bytes.len() != 8 {
			Err(Error::InvalidLength)?
		}

		Ok(ViewId(bytes.as_slice().try_into().unwrap()))
	}
}

impl Serialize for ViewId {
	fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

struct ViewIdVisitor;

impl<'de> Visitor<'de> for ViewIdVisitor {
	type Value = ViewId;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("expecting a hex-encoded array of 8 bytes")
	}

	fn visit_str<E>(self, v: &str) -> std::prelude::v1::Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		let view_id: Result<ViewId> = v.parse();

		view_id.map_err(|e| E::custom(format!("failed to parse ViewId: {}", e.to_string())))
	}
}

impl<'de> Deserialize<'de> for ViewId {
	fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_str(ViewIdVisitor)
	}
}
