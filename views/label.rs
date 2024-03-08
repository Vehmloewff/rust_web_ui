use crate::{state::GenericElement, Context, View, ViewId, ViewResult};

#[derive(Debug)]
pub struct Label {
	label: Option<String>,
	view_id: Option<ViewId>,
}

impl Label {
	pub fn new(label: impl Into<String>) -> Label {
		Label {
			label: Some(label.into()),
			view_id: None,
		}
	}

	pub fn set_label(&mut self, new_label: impl Into<String>) {
		self.label = Some(new_label.into())
	}
}

impl View for Label {
	fn update(&mut self, context: &mut Context) -> ViewResult<()> {
		if let None = self.view_id {
			self.view_id = Some(ViewId::generate());

			let mut element = GenericElement::new("span");
			element.id = self.view_id.clone();
			element.text_content = self.label.take();

			context.push_child(element)
		}

		if let Some(label) = self.label.take() {}

		Ok(())
	}
}

// pub trait LabelExt {
// 	fn label(&mut self, text: &str) {
// 		self.text
// 	}
// }

// fn hello(ui: &mut Ui, name: &str) {
// 	let count = ui.state(|| 0);

// 	ui.container(|ui, ctx| {
// 		ctx.gap(23);

// 		if ctx.clicked() {
// 			self.count += 1;
// 		}

// 		ui.label(self.count.into());
// 	});

// 	ui.container()
// 		.color(Colors::Red)
// 		.child(ui.column().children(vec![ui.child(), ui.child(), ui.child()]))
// }
