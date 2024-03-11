use crate::{State, Ui, ViewId, Window};
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
	pub attributes: Vec<(String, String)>,
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
		name: String,
		value: Option<String>,
	},
}

#[derive(Debug)]
pub enum Node {
	Element(Element),
	New,
}

impl Node {
	pub fn element(&self) -> Option<&Element> {
		match self {
			Node::Element(element) => Some(element),
			Node::New => None,
		}
	}

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
	pub fn child<'a, T>(&'a mut self, key: &str, window: &'a mut Window, _widget: T) -> Ui<'a, T> {
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

		Ui::new(node, state, window)
	}

	pub fn get_updates(&mut self, updates: &mut Vec<ElementUpdate>) {
		let current_diff = match self.current_diff.replace(ElementDiff::new()) {
			Some(diff) => diff,
			None => return,
		};

		self.get_attribute_updates(updates, current_diff.touched_attributes);
		self.get_children_updates(updates, current_diff.touched_keys);
	}

	fn get_attribute_updates(&mut self, updates: &mut Vec<ElementUpdate>, set_names: HashMap<String, bool>) {
		let mut to_remove = Vec::new();

		for (name, _) in &self.attributes {
			if !set_names.contains_key(name) {
				to_remove.push(name.clone())
			}
		}

		for name in to_remove {
			self.attributes.remove(&name);

			updates.push(ElementUpdate::SetAttribute {
				element_id: self.id.clone(),
				name,
				value: None,
			})
		}

		for (name, was_updated) in set_names {
			if was_updated {
				let value = self.attributes.get(&name).unwrap().clone();

				updates.push(ElementUpdate::SetAttribute {
					element_id: self.id.clone(),
					name,
					value: Some(value),
				})
			}
		}
	}

	fn get_children_updates(&mut self, updates: &mut Vec<ElementUpdate>, touched_keys: HashSet<String>) {
		let mut keys_to_remove = Vec::new();

		for (key, _) in &self.keyed_children {
			if !touched_keys.contains(key) {
				keys_to_remove.push(key.to_owned())
			}
		}

		for key in keys_to_remove {
			self.keyed_children.remove(&key);

			updates.push(ElementUpdate::RemoveChild {
				element_id: self.id.clone(),
				key,
			})
		}

		for key in touched_keys {
			let ElementChild { index, node, .. } = self.keyed_children.get_mut(&key).unwrap();

			let element = match node {
				Node::Element(element) => element,
				Node::New => continue,
			};

			let is_new = self.current_diff.is_none();

			if is_new {
				updates.push(ElementUpdate::MakeChild {
					element_id: element.id.clone(),
					key,
					index: index.clone(),
					element: element.get_repr(),
				});

				element.start_diffing();
			} else {
				element.get_updates(updates);
			}
		}
	}

	pub fn get_repr(&self) -> ElementRepr {
		let mut children = Vec::new();
		let mut attributes = Vec::new();

		for (name, value) in &self.attributes {
			attributes.push((name.clone(), value.clone()));
		}

		for (key, ElementChild { index, node, .. }) in &self.keyed_children {
			let element = match node {
				Node::Element(element) => element,
				Node::New => continue,
			};

			children.push(ElementChildRepr {
				key: key.clone(),
				index: index.clone(),
				element: element.get_repr(),
			})
		}

		ElementRepr {
			id: self.id.clone(),
			tag: self.tag.clone(),
			children,
			attributes,
		}
	}

	pub fn start_diffing(&mut self) {
		self.current_diff.replace(ElementDiff::new());

		for (_, child) in &mut self.keyed_children {
			match &mut child.node {
				Node::Element(element) => element.start_diffing(),
				Node::New => (),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{Button, Ctx, Label, StatefulWidget};

	use super::*;
	use pretty_assertions::assert_eq;

	struct Counter;

	struct CounterState {
		count: usize,
	}

	impl Default for CounterState {
		fn default() -> Self {
			CounterState { count: 0 }
		}
	}

	impl StatefulWidget<'_> for Counter {
		type Props = ();
		type State = CounterState;

		fn render(mut ctx: Ctx<'_>, _: Self::Props, state: &mut Self::State) {
			let CounterState { count } = state;

			ctx.child("decrement_button", Button).run(|props| {
				props.label("Decrement");
			});

			let label = format!("current count is {count}");
			ctx.child("label", Label).run(|props| {
				props.text(&label);
			});

			ctx.child("increment_button", Button).run(|props| {
				props.label("Increment");
			});

			*count += 1;
		}
	}

	// #[test]
	// fn build_repr() {
	// 	let mut root = Element::new("div");
	// 	let mut ctx = Ctx::new();

	// 	counter.render(&mut root, &mut ctx);

	// 	let repr = Element::get_repr(&mut root);

	// 	assert_eq!(repr.tag, "div");
	// 	assert_eq!(repr.children.iter().find(|item| item.key == "decrement_button").unwrap().index, 0);
	// 	assert_eq!(repr.children.iter().find(|item| item.key == "label").unwrap().index, 1);
	// 	assert_eq!(repr.children.iter().find(|item| item.key == "increment_button").unwrap().index, 2);
	// }

	// #[test]
	// fn compute_diffs_after_render() {
	// 	let mut root = Element::new("div");
	// 	let mut ctx = Ctx::new();

	// 	let mut counter = Counter { count: 0 };

	// 	// render the counter once just so that we can check the validity of a pure diff, where only one thing should change
	// 	counter.render(&mut root, &mut ctx);
	// 	Element::start_diffing(&mut root);

	// 	// updated and diff
	// 	let mut updates = Vec::new();
	// 	counter.render(&mut root, &mut ctx);
	// 	Element::get_updates(&mut root, &mut updates);

	// 	assert_eq!(updates.len(), 1);
	// 	assert_eq!(
	// 		updates.get(0).unwrap(),
	// 		&ElementUpdate::SetAttribute {
	// 			element_id: root.keyed_children.get("label").unwrap().node.element().unwrap().id.clone(),
	// 			name: "__textContent".into(),
	// 			value: Some("current count is 1".into())
	// 		}
	// 	);

	// 	println!("after this:::");

	// 	// Update and diff a second time. Due to the un-pure nature of diffing, we need to make sure we can produce the same result twice
	// 	let mut updates = Vec::new();
	// 	counter.render(&mut root, &mut ctx);
	// 	Element::get_updates(&mut root, &mut updates);

	// 	dbg!(&updates);

	// 	assert_eq!(updates.len(), 1);
	// 	assert_eq!(
	// 		updates.get(0).unwrap(),
	// 		&ElementUpdate::SetAttribute {
	// 			element_id: root.keyed_children.get("label").unwrap().node.element().unwrap().id.clone(),
	// 			name: "__textContent".into(),
	// 			value: Some("current count is 2".into())
	// 		}
	// 	);
	// }
}
