use crate::{style::Theme, Element, ElementRepr, Ui, Window};
use std::fmt::{self, Display};

pub struct View {
	root: Element,
	window: Window,
}

impl View {
	pub fn new(title: impl Into<String>, path: impl Into<String>, theme: Theme) -> View {
		View {
			root: Element::new("body"),
			window: Window::new(title, path, theme),
		}
	}

	pub fn define_root<T>(&mut self, key: &str, _widget: T) -> Ui<'_, T> {
		let (node, state) = self.root.child(key);

		Ui::new(node, state, &mut self.window)
	}

	pub fn get_repr(&self) -> ElementRepr {
		self.root.get_repr()
	}
}

impl Display for View {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<!DOCTYPE html><html lang=\"en\">")?;

		self.window.write_html_head(f)?;
		self.get_repr().write_html(f, None)?;

		write!(f, "</html>")?;

		Ok(())
	}
}
