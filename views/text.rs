use crate::{state::GenericElement, Context, View, ViewId, ViewResult};

#[derive(Debug)]
pub struct Text {
	label: Option<String>,
	view_id: Option<ViewId>,
}

impl Text {
	pub fn new(label: impl Into<String>) -> Text {
		Text {
			label: Some(label.into()),
			view_id: None,
		}
	}
}

impl View for Text {
	fn update(&mut self, context: &mut Context) -> ViewResult<()> {
		if let None = self.view_id {
			self.view_id = Some(ViewId::generate());

			let mut element = GenericElement::new("span");
			element.id = self.view_id.clone();
			element.text_content = self.label.take();

			context.push_child(element)
		}

		Ok(())
	}
}
