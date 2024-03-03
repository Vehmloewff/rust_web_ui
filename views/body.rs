use super::{Context, NodeError, View};

#[derive(Debug)]
pub struct Body<T: View> {
	child: T,
}

impl<T: View> View for Body<T> {
	fn update(&mut self, context: &mut Context) -> Result<(), NodeError> {
		self.child.update(context)?;

		Ok(())
	}
}
