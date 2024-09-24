use crate::ViewId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

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
	pub fn write_css(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut size_media = HashMap::new();
		let mut dark_media = HashMap::new();
		let mut normal_rules = HashMap::new();

		for (key, value) in &self.attributes {
			if !key.starts_with("style:") {
				continue;
			}

			let mut sections = key.split(':').collect::<Vec<_>>();
			let property = sections.pop().unwrap();
			let mut css = format!("{{ {property}: {value}; }}");
			let mut selector = format!("#w{}", self.id);

			for section in sections {
				if section == "hover" {
					selector.push_str(":hover");
					continue;
				}

				if section == "active" {
					selector.push_str(":active");
					continue;
				}

				if section == "focus" {
					selector.push_str(":focus");
					continue;
				}

				if section.starts_with("screen-") {
					// format!("min-width: {}px", size)
					let size: usize = section[7..].parse().unwrap();
					size_media.insert(size, HashMap());

					continue;
				}

				if section == "dark" {
					media_statements.push("prefers-color-scheme: dark".into());
					continue;
				}
			}

			css = format!("{selector} {css}");

			for media in media_statements {
				css = format!("@media ({media}) {{ {css} }}")
			}
		}

		// write!(f, "{}", css)?;

		for ElementChildRepr { element, .. } in &self.children {
			element.write_css(f)?;
		}

		Ok(())
	}

	pub fn write_html(&self, f: &mut fmt::Formatter, key: Option<&str>) -> fmt::Result {
		let tag = &self.tag;
		let id = &self.id;
		let mut inner = None;

		write!(f, "<{tag} id=\"w{id}\"")?;

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

struct CssRuleSet {
	rules: HashMap<String, Vec<String>>,
}

struct CssTracker {
	size_media: HashMap<usize, CssRuleSet>,
	dark_media: CssRuleSet,
	normal_rules: CssRuleSet,
}
