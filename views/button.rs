use crate::{
	state::{ActionDefinition, ActionLoaderBehavior},
	Context, NodeError, View, ViewId,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Button<T: View> {
	child: T,
	action_id: Option<ViewId>,
	show_loader: bool,
	was_clicked: bool,
}

impl<T: View> Button<T> {
	pub fn new(child: T) -> Button<T> {
		Button {
			child,
			action_id: None,
			show_loader: false,
			was_clicked: false,
		}
	}

	pub fn set_loader_preference(&mut self, show: bool) {
		self.show_loader = show
	}

	pub fn get_child(&self) -> &T {
		&self.child
	}

	pub fn get_mut(&mut self) -> &mut T {
		&mut self.child
	}

	pub fn was_clicked(&self) -> bool {
		self.was_clicked
	}

	pub fn cancel_loader(&self, context: &mut Context) {}
}

impl<T: View> View for Button<T> {
	fn update(&mut self, context: &mut Context) -> Result<(), NodeError> {
		self.child.update(context)?;
		self.was_clicked = false;

		if let Some(action_id) = &self.action_id {
			context.deny_children()?;

			if let Some(_) = context.take_action(action_id) {
				self.was_clicked = true;
			}
		} else {
			let action_id = ViewId::generate();
			let element = context
				.reduce_children("button")
				.action(ActionDefinition::new(action_id.clone(), "click").loader_behavior(ActionLoaderBehavior::Show));

			context.push_child(element);
			self.action_id.replace(action_id);
		}

		Ok(())
	}
}
