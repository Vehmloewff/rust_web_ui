use crate::{ctx::Ctx, Button, ViewId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum NodeRepr {
	Element(ElementRepr),
	Text(TextRepr),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct TextRepr {
	pub text: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ElementChildRepr {
	pub key: String,
	pub index: usize,
	pub node: NodeRepr,
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
		node: NodeRepr,
	},
}

#[derive(Debug)]
pub enum Node {
	Element(Element),
	Text(Text),
	New,
}

impl Node {
	fn to_repr(&self) -> NodeRepr {
		match self {
			Node::Element(element) => NodeRepr::Element(Element::get_repr(element)),
			Node::Text(Text { text, .. }) => NodeRepr::Text(TextRepr { text: text.clone() }),
			Node::New => NodeRepr::Text(TextRepr { text: String::new() }),
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

	pub fn set_text(&mut self, string: &str) {
		if let Node::Text(text) = self {
			text.set(string)
		} else {
			*self = Node::Text(Text::new(string))
		}
	}
}

impl Node {
	pub fn start_diffing(node: &mut Node) {
		match node {
			Node::Element(element) => Element::start_diffing(element),
			Node::Text(text) => Text::start_diffing(text),
			Node::New => (),
		}
	}
}

pub struct Ui<'a> {
	node: &'a mut Node,
	context: &'a mut Ctx,
}

impl Ui<'_> {
	pub fn text(&mut self, text: &str) {
		self.node.set_text(text)
	}

	pub fn include<'a, T: UiBuilder<'a>>(&'a mut self) -> T {
		T::new(self.node, self.context)
	}

	pub fn button(&mut self) -> Button<'_> {
		self.include::<Button>()
	}
}

#[derive(Debug)]
struct TextDiff {
	was_changed: bool,
}

impl TextDiff {
	pub fn new() -> TextDiff {
		TextDiff { was_changed: false }
	}
}

#[derive(Debug)]
pub struct Text {
	text: String,
	current_diff: Option<TextDiff>,
}

impl Text {
	pub fn new(text: impl Into<String>) -> Text {
		Text {
			text: text.into(),
			current_diff: None,
		}
	}

	pub fn set(&mut self, text: &str) {
		if self.text != text {
			self.text = text.to_string();

			if let Some(diff) = &mut self.current_diff {
				diff.was_changed = true;
			}
		}
	}
}

impl Text {
	pub fn start_diffing(text: &mut Text) {
		text.current_diff.replace(TextDiff::new());
	}

	pub fn should_make(text: &mut Text) -> bool {
		match text.current_diff.replace(TextDiff::new()) {
			Some(diff) => diff.was_changed,
			None => false,
		}
	}
}

#[derive(Debug)]
enum TouchedChildKind {
	Created,
	WasText,
	WasElement,
}

#[derive(Debug)]
struct ElementDiff {
	touched_keys: HashMap<String, TouchedChildKind>,
}

impl ElementDiff {
	pub fn new() -> ElementDiff {
		ElementDiff { touched_keys: HashMap::new() }
	}
}

#[derive(Debug)]
pub struct Element {
	id: ViewId,
	tag: String,
	current_diff: Option<ElementDiff>,
	keyed_children: HashMap<String, (usize, Node)>,
}

impl Element {
	pub fn new(tag: impl Into<String>) -> Element {
		Element {
			id: ViewId::generate(),
			tag: tag.into(),
			current_diff: None,
			keyed_children: HashMap::new(),
		}
	}

	fn remember_touch(&mut self, key: &str, kind: TouchedChildKind) {
		if let Some(diff) = &mut self.current_diff {
			diff.touched_keys.insert(key.to_owned(), kind);
		}
	}
}

impl Element {
	pub fn child<'a>(&'a mut self, key: &str, context: &'a mut Ctx) -> Ui<'a> {
		let has_key = self.keyed_children.contains_key(key);

		if !has_key {
			self.keyed_children.insert(key.to_owned(), (self.keyed_children.len(), Node::New));
		}

		if let Some((_, node)) = self.keyed_children.get(key) {
			self.remember_touch(
				key,
				match &node {
					Node::Element(_) => TouchedChildKind::WasElement,
					Node::Text(_) => TouchedChildKind::WasText,
					Node::New => TouchedChildKind::Created,
				},
			);
		};

		let (_, node) = self.keyed_children.get_mut(key).unwrap();

		Ui { context, node }
	}
}

impl Element {
	pub fn get_updates(element: &mut Element, updates: &mut Vec<ElementUpdate>) {
		dbg!(&element);
		let current_diff = match element.current_diff.replace(ElementDiff::new()) {
			Some(diff) => diff,
			None => return,
		};

		let mut keys_to_remove = Vec::new();

		for (key, _) in &element.keyed_children {
			if !current_diff.touched_keys.contains_key(key) {
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

		for (key, kind) in current_diff.touched_keys {
			let (index, node) = element.keyed_children.get_mut(&key).unwrap();

			let did_make = match &kind {
				TouchedChildKind::Created => true,
				TouchedChildKind::WasElement => match node {
					Node::Element(_) => false,
					_ => true,
				},
				TouchedChildKind::WasText => match node {
					Node::Text(text) => Text::should_make(text),
					_ => true,
				},
			};

			if did_make {
				updates.push(ElementUpdate::MakeChild {
					element_id: element.id.clone(),
					key,
					index: index.clone(),
					node: node.to_repr(),
				})
			}

			if let Node::Element(element) = node {
				Element::get_updates(element, updates)
			}
		}
	}

	pub fn get_repr(element: &Element) -> ElementRepr {
		let mut children = Vec::new();

		for (key, (index, child)) in &element.keyed_children {
			children.push(ElementChildRepr {
				key: key.clone(),
				index: index.clone(),
				node: child.to_repr(),
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

		for (_, (_, child)) in &mut element.keyed_children {
			Node::start_diffing(child)
		}
	}
}

pub trait UiBuilder<'a> {
	fn new(node: &'a mut Node, context: &'a mut Ctx) -> Self;
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;

	struct Counter {
		count: usize,
	}

	impl Counter {
		pub fn render(&mut self, el: &mut Element, ctx: &mut Ctx) {
			el.child("decrement_button", ctx).button().label("decrement");
			el.child("label", ctx).text(&format!("current count is {}", &self.count));
			el.child("increment_button", ctx).button().label("increment");

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
		assert_eq!(
			updates.get(0).unwrap(),
			&ElementUpdate::MakeChild {
				element_id: root.id.clone(),
				key: "label".into(),
				index: 1,
				node: NodeRepr::Text(TextRepr {
					text: "current count is 1".into()
				})
			}
		);

		println!("after this:::");

		// Update and diff a second time. Due to the un-pure nature of diffing, we need to make sure we can produce the same result twice
		let mut updates = Vec::new();
		counter.render(&mut root, &mut ctx);
		Element::get_updates(&mut root, &mut updates);

		dbg!(&updates);

		assert_eq!(updates.len(), 1);
		assert_eq!(
			updates.get(0).unwrap(),
			&ElementUpdate::MakeChild {
				element_id: root.id,
				key: "label".into(),
				index: 1,
				node: NodeRepr::Text(TextRepr {
					text: "current count is 2".into()
				})
			}
		);
	}
}
