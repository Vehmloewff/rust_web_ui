use crate::{ctx::Ctx, Element, Node, UiBuilder};

pub struct Button<'a> {
	element: &'a mut Element,
	context: &'a mut Ctx,
}

impl Button<'_> {
	pub fn label(&mut self, text: &str) {
		self.element.child("text", self.context).text(text);
	}
}

impl<'a> UiBuilder<'a> for Button<'a> {
	fn new(node: &'a mut Node, context: &'a mut Ctx) -> Button<'a> {
		Button {
			element: node.get_element(|| Element::new("button")),
			context,
		}
	}
}
