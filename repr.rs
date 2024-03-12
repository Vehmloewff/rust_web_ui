use crate::ViewId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ElementChildRepr {
	pub key: String,
	pub element: ElementRepr,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ElementRepr {
	pub id: ViewId,
	pub tag: String,
	pub children: Vec<ElementChildRepr>,
	pub attributes: Vec<(String, String)>,
}

impl ElementRepr {
	pub fn write_html(&self, f: &mut fmt::Formatter, key: Option<&str>) -> fmt::Result {
		let tag = &self.tag;
		let id = &self.id;
		let mut inner = None;

		write!(f, "<{tag} id=\"{id}\"")?;

		if let Some(key) = key {
			write!(f, " child-key=\"{key}\"")?;
		}

		for (name, value) in &self.attributes {
			if name == "__textContent" {
				inner = Some(value)
			} else {
				write!(f, " {name}=\"{value}\"")?;
			}
		}

		// TODO actions:
		//
		// for action in &self.actions {
		// 	let event = &action.event;
		// 	let id = &action.id;
		// 	let kind = to_string(&action.payload_kind).unwrap().replace('"', "\'");
		//
		// 	write!(f, " on{event}=\"window.triggerAction('{id}', event, {kind})\"")?;
		// }

		write!(f, ">")?;

		if let Some(html) = inner {
			write!(f, "{html}")?;
		}

		// TODO sort by index
		for ElementChildRepr { element, key, .. } in &self.children {
			element.write_html(f, Some(key))?;
		}

		write!(f, "</{tag}>")?;

		Ok(())
	}
}
