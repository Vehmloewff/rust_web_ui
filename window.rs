use std::fmt;

pub enum WindowUpdate {
	SetTitle { new_title: String },
}

pub enum UpstreamWindowUpdate {
	PathChanged { new_path: String },
}

pub struct Window {
	title: String,
	path: String,
	path_did_change: bool,
	updates: Vec<WindowUpdate>,
}

impl Window {
	pub fn new(title: impl Into<String>, path: impl Into<String>) -> Window {
		Window {
			title: title.into(),
			path: path.into(),
			path_did_change: true,
			updates: Vec::new(),
		}
	}

	pub fn get_path_change(&self) -> Option<&str> {
		if self.path_did_change {
			Some(&self.path)
		} else {
			None
		}
	}

	pub fn set_title(&mut self, title: String) {
		self.title = title.clone();
		self.updates.push(WindowUpdate::SetTitle { new_title: title });
	}

	pub fn take_updates(&mut self) -> Vec<WindowUpdate> {
		self.updates.drain(..).collect()
	}

	pub fn apply_update(&mut self, update: UpstreamWindowUpdate) {
		self.path_did_change = true;

		match update {
			UpstreamWindowUpdate::PathChanged { new_path } => self.path = new_path,
		}
	}

	pub fn write_html_head(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<title>{}</title>", self.title)?;

		Ok(())
	}
}
