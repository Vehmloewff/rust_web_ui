use std::any::Any;

#[derive(Debug)]
pub struct State(Option<Box<dyn Any>>);

impl State {
	pub fn new() -> State {
		State(None)
	}

	pub unsafe fn get<T: Default + 'static>(&mut self) -> &mut T {
		if let None = self.0 {
			self.0.replace(Box::new(T::default()));
		}

		let any = self.0.as_mut().unwrap_unchecked();
		let any_ptr = any.as_mut() as *mut dyn Any;
		let t_ptr = any_ptr as *mut T;

		&mut *t_ptr
	}
}
