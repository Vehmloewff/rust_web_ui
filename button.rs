use std::default;

use crate::{ctx::Ctx, Element, Node, Widget};

// pub struct Button<'a> {
// 	element: &'a mut Element,
// 	context: &'a mut Ctx,
// }

// impl Button<'_> {
// 	pub fn label(&mut self, text: &str) {
// 		self.element.child("text", self.context).text(text);
// 	}
// }

// impl<'a> UiBuilder<'a> for Button<'a> {
// 	type State = ();

// 	fn new(node: &'a mut Node, _: &mut (), context: &'a mut Ctx) -> Button<'a> {
// 		Button {
// 			element: node.get_element(|| Element::new("button")),
// 			context,
// 		}
// 	}
// }

pub struct Button;

pub struct ButtonProps<'a> {
	label: &'a str,
}

impl<'a> Default for ButtonProps<'a> {
	fn default() -> Self {
		Self { label: "click me" }
	}
}

impl<'a> Widget<'a> for Button {
	type Props = ButtonProps<'a>;

	fn provide_tag() -> String {
		"button".into()
	}

	fn render(el: &mut Element, ctx: &mut Ctx, props: Self::Props) {
		// TODO here
	}
}
