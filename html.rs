use crate::{ElementChildRepr, ElementRepr};
use std::fmt;

pub trait Html {
	fn write_html(&self, f: &mut fmt::Formatter, key: &str) -> fmt::Result;
}

impl Html for ElementRepr {
	fn write_html(&self, f: &mut fmt::Formatter, key: &str) -> fmt::Result {
		let tag = &self.tag;
		let id = &self.id;

		write!(f, "<{tag} child-key=\"{key}\" id=\"{id}\"")?;

		for (name, value) in &self.attributes {
			write!(f, " {name}=\"{value}\"")?;
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

		// TODO sort by index
		for ElementChildRepr { element, key, .. } in &self.children {
			element.write_html(f, key)?;
		}

		write!(f, "</{tag}>")?;

		Ok(())
	}
}
