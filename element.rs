use crate::{ctx::Ctx, State, Ui, ViewId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ElementChildRepr {
	pub key: String,
	pub index: usize,
	pub element: ElementRepr,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ElementRepr {
	pub id: ViewId,
	pub tag: String,
	pub children: Vec<ElementChildRepr>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum ElementUpdate {
	RemoveChild {
		element_id: ViewId,
		key: String,
	},
	MakeChild {
		element_id: ViewId,
		key: String,
		index: usize,
		element: ElementRepr,
	},
	SetAttribute {
		element_id: ViewId,
		key: String,
		value: Option<String>,
	},
}

#[derive(Debug)]
pub enum Node {
	Element(Element),
	New,
}

impl Node {
	pub fn get_element(&mut self, creator: impl FnOnce() -> Element) -> &mut Element {
		if let Node::Element(element) = self {
			return element;
		}

		*self = Node::Element(creator());

		match self {
			Node::Element(element) => element,
			_ => panic!("previous logic failed"),
		}
	}
}

impl Node {
	pub fn start_diffing(node: &mut Node) {
		match node {
			Node::Element(element) => Element::start_diffing(element),
			Node::New => (),
		}
	}
}

#[derive(Debug)]
struct ElementDiff {
	touched_keys: HashSet<String>,
	touched_attributes: HashMap<String, bool>,
}

impl ElementDiff {
	pub fn new() -> ElementDiff {
		ElementDiff {
			touched_keys: HashSet::new(),
			touched_attributes: HashMap::new(),
		}
	}
}

#[derive(Debug)]
struct ElementChild {
	index: usize,
	node: Node,
	state: State,
}

#[derive(Debug)]
pub struct Element {
	id: ViewId,
	tag: String,
	current_diff: Option<ElementDiff>,
	keyed_children: HashMap<String, ElementChild>,
	attributes: HashMap<String, String>,
}

impl Element {
	pub fn new(tag: impl Into<String>) -> Element {
		Element {
			id: ViewId::generate(),
			tag: tag.into(),
			current_diff: None,
			keyed_children: HashMap::new(),
			attributes: HashMap::new(),
		}
	}

	pub fn set_attribute(&mut self, name: &str, value: &str) {
		let should_set = match self.attributes.get(name) {
			Some(old_value) => old_value != value,
			None => true,
		};

		if should_set {
			self.attributes.insert(name.to_string(), value.to_string());
		}

		if let Some(diff) = &mut self.current_diff {
			diff.touched_attributes.insert(name.to_owned(), should_set);
		}
	}
}

impl Element {
	pub fn child<'a, T>(&'a mut self, key: &str, context: &'a mut Ctx, _widget: T) -> Ui<'a, T> {
		let has_key = self.keyed_children.contains_key(key);

		if !has_key {
			self.keyed_children.insert(
				key.to_owned(),
				ElementChild {
					index: self.keyed_children.len(),
					node: Node::New,
					state: State::new(),
				},
			);
		}

		if let Some(diff) = &mut self.current_diff {
			diff.touched_keys.insert(key.to_string());
		}

		let ElementChild { node, state, .. } = self.keyed_children.get_mut(key).unwrap();

		Ui::new(node, state, context)
	}
}

impl Element {
	pub fn get_updates(element: &mut Element, updates: &mut Vec<ElementUpdate>) {
		let current_diff = match element.current_diff.replace(ElementDiff::new()) {
			Some(diff) => diff,
			None => return,
		};

		Element::get_attribute_updates(element, updates, current_diff.touched_attributes);
		Element::get_children_updates(element, updates, current_diff.touched_keys);
	}

	fn get_attribute_updates(element: &mut Element, updates: &mut Vec<ElementUpdate>, set_names: HashMap<String, bool>) {
		let mut to_remove = Vec::new();

		for (name, _) in &element.attributes {
			if !set_names.contains_key(name) {
				to_remove.push(name.clone())
			}
		}

		for name in to_remove {
			element.attributes.remove(&name);

			updates.push(ElementUpdate::SetAttribute {
				element_id: element.id.clone(),
				key: name,
				value: None,
			})
		}

		for (name, was_updated) in set_names {
			if was_updated {
				let value = element.attributes.get(&name).unwrap().clone();

				updates.push(ElementUpdate::SetAttribute {
					element_id: element.id.clone(),
					key: name,
					value: Some(value),
				})
			}
		}
	}

	fn get_children_updates(element: &mut Element, updates: &mut Vec<ElementUpdate>, touched_keys: HashSet<String>) {
		let mut keys_to_remove = Vec::new();

		for (key, _) in &element.keyed_children {
			if !touched_keys.contains(key) {
				keys_to_remove.push(key.to_owned())
			}
		}

		for key in keys_to_remove {
			element.keyed_children.remove(&key);

			updates.push(ElementUpdate::RemoveChild {
				element_id: element.id.clone(),
				key,
			})
		}

		for key in touched_keys {
			let ElementChild { index, node, .. } = element.keyed_children.get_mut(&key).unwrap();

			let element = match node {
				Node::Element(element) => element,
				Node::New => continue,
			};

			let is_new = element.current_diff.is_none();

			if is_new {
				updates.push(ElementUpdate::MakeChild {
					element_id: element.id.clone(),
					key,
					index: index.clone(),
					element: Element::get_repr(&element),
				});

				Element::start_diffing(element);
			} else {
				Element::get_updates(element, updates)
			}
		}
	}

	pub fn get_repr(element: &Element) -> ElementRepr {
		let mut children = Vec::new();

		for (key, ElementChild { index, node, .. }) in &element.keyed_children {
			let element = match node {
				Node::Element(element) => element,
				Node::New => continue,
			};

			children.push(ElementChildRepr {
				key: key.clone(),
				index: index.clone(),
				element: Element::get_repr(element),
			})
		}

		ElementRepr {
			id: element.id.clone(),
			tag: element.tag.clone(),
			children,
		}
	}

	pub fn start_diffing(element: &mut Element) {
		element.current_diff.replace(ElementDiff::new());

		for (_, child) in &mut element.keyed_children {
			Node::start_diffing(&mut child.node)
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::Button;

	use super::*;
	use pretty_assertions::assert_eq;

	struct Counter {
		count: usize,
	}

	impl Counter {
		fn render(&mut self, el: &mut Element, ctx: &mut Ctx) {
			el.child("decrement_button", ctx, Button); // .label("increment");
			el.child("label", ctx, Button); // .text(&format!("current count is {}", &self.count));
			el.child("increment_button", ctx, Button); //.button().label("increment");

			self.count += 1;
		}
	}

	#[test]
	fn build_repr() {
		let mut root = Element::new("div");
		let mut ctx = Ctx::new();

		let mut counter = Counter { count: 0 };
		counter.render(&mut root, &mut ctx);

		let repr = Element::get_repr(&mut root);

		assert_eq!(repr.tag, "div");
		assert_eq!(repr.children.iter().find(|item| item.key == "decrement_button").unwrap().index, 0);
		assert_eq!(repr.children.iter().find(|item| item.key == "label").unwrap().index, 1);
		assert_eq!(repr.children.iter().find(|item| item.key == "increment_button").unwrap().index, 2);
	}

	#[test]
	fn compute_diffs_after_render() {
		let mut root = Element::new("div");
		let mut ctx = Ctx::new();

		let mut counter = Counter { count: 0 };

		// render the counter once just so that we can check the validity of a pure diff, where only one thing should change
		counter.render(&mut root, &mut ctx);
		Element::start_diffing(&mut root);

		// updated and diff
		let mut updates = Vec::new();
		counter.render(&mut root, &mut ctx);
		Element::get_updates(&mut root, &mut updates);

		assert_eq!(updates.len(), 1);
		// assert_eq!(
		// 	updates.get(0).unwrap(),
		// 	&ElementUpdate::SetAttribute {
		// 		element_id: root.id.clone(),
		// 		key: "label".into(),
		// 		index: 1,
		// 	}
		// );

		println!("after this:::");

		// Update and diff a second time. Due to the un-pure nature of diffing, we need to make sure we can produce the same result twice
		let mut updates = Vec::new();
		counter.render(&mut root, &mut ctx);
		Element::get_updates(&mut root, &mut updates);

		dbg!(&updates);

		assert_eq!(updates.len(), 1);
		// assert_eq!(
		// 	updates.get(0).unwrap(),
		// 	&ElementUpdate::MakeChild {
		// 		element_id: root.id,
		// 		key: "label".into(),
		// 		index: 1,
		// 		node: NodeRepr::Text(TextRepr {
		// 			text: "current count is 2".into()
		// 		})
		// 	}
		// );
	}
}
