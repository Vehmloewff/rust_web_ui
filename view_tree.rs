use crate::{
	state::{ActionDefinition, DownstreamMessage, GenericElement},
	ViewId,
};
use error_stack::ResultExt;
use rutils::errors;
use serde_json::to_string;
use std::{
	collections::HashMap,
	fmt::{self, Display, Write},
	time::{Duration, Instant},
};

errors!(CssNotExists, ElementNotExists);

struct PhysicalTreeNode {
	attributes: HashMap<String, String>,
	actions: Vec<ActionDefinition>,
	children: Vec<TreeNode>,
	delete_at: Option<Instant>,
	text_content: Option<String>,
	tag: String,
}

impl PhysicalTreeNode {
	fn write_opening_tag(&self, f: &mut fmt::Formatter, id: Option<&ViewId>) -> fmt::Result {
		let tag = &self.tag;
		write!(f, "<{tag}")?;

		if let Some(id) = id {
			write!(f, " id=\"{id}\"")?;
		}

		for (name, value) in &self.attributes {
			write!(f, " {name}=\"{value}\"")?;
		}

		for action in &self.actions {
			let event = &action.event;
			let id = &action.id;
			let kind = to_string(&action.payload_kind).unwrap().replace('"', "\'");

			write!(f, " on{event}=\"window.triggerAction('{id}', event, {kind})\"")?;
		}

		write!(f, ">")?;

		Ok(())
	}

	fn write_closing_tag(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let tag = &self.tag;

		write!(f, "</{tag}>")
	}

	fn should_show(&self) -> bool {
		if let Some(instant) = self.delete_at {
			instant.checked_duration_since(Instant::now()).is_some()
		} else {
			true
		}
	}
}

enum TreeNode {
	Physical(PhysicalTreeNode),
	Reference(ViewId),
}

pub struct ViewTree {
	nodes: HashMap<ViewId, PhysicalTreeNode>,
	root_nodes: Vec<TreeNode>,
	title: Option<String>,
	path: String,
	errors: Vec<String>,
	css: HashMap<ViewId, String>,
	lang: String,
	head_tags: Vec<String>,
}

impl ViewTree {
	pub fn new(initial_path: impl Into<String>) -> ViewTree {
		ViewTree {
			nodes: HashMap::new(),
			root_nodes: Vec::new(),
			title: None,
			path: initial_path.into(),
			errors: Vec::new(),
			css: HashMap::new(),
			lang: "en".into(),
			head_tags: Vec::new(),
		}
	}

	fn get_element(&mut self, id: &ViewId) -> Result<&mut PhysicalTreeNode> {
		Ok(match self.nodes.get_mut(id) {
			Some(element) => element,
			None => Err(Error::ElementNotExists).attach_printable_lazy(|| format!("tried to get element with id '{id}'"))?,
		})
	}

	fn map_elements(&mut self, elements: Vec<GenericElement>) -> Vec<TreeNode> {
		let mut nodes = Vec::new();

		for element in elements {
			let mut attributes = HashMap::new();

			for (name, value) in element.attributes {
				attributes.insert(name, value);
			}

			let physical = PhysicalTreeNode {
				actions: element.actions,
				attributes,
				tag: element.tag,
				children: self.map_elements(element.children),
				text_content: element.text_content,
				delete_at: match element.delete_after_ms {
					Some(ms) => Some(Instant::now() + Duration::from_millis(ms as u64)),
					None => None,
				},
			};

			nodes.push(match element.id {
				Some(id) => {
					self.nodes.insert(id.clone(), physical);
					TreeNode::Reference(id)
				}
				None => TreeNode::Physical(physical),
			})
		}

		nodes
	}

	fn drop_dependencies(&mut self, nodes: Vec<TreeNode>) {
		for node in nodes {
			match node {
				TreeNode::Physical(PhysicalTreeNode { children, .. }) => self.drop_dependencies(children),
				TreeNode::Reference(id) => {
					if let Some(physical) = self.nodes.remove(&id) {
						self.drop_dependencies(physical.children)
					}
				}
			}
		}
	}

	pub fn add_head_defaults(&mut self, js_path: &str) {
		self.head_tags.push("<meta charset=\"UTF-8\">".into());
		self.head_tags
			.push("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">".into());
		self.head_tags.push(format!("<script src=\"{js_path}\" async defer></script>"));
	}

	pub fn apply_downstream_messages(&mut self, messages: Vec<DownstreamMessage>) -> Result<()> {
		dbg!(&messages);
		for message in messages {
			match message {
				DownstreamMessage::AddCssChunk { id, content } => {
					self.css.insert(id, content);
				}
				DownstreamMessage::DeleteCssChunk { id } => {
					if let None = self.css.remove(&id) {
						Err(Error::CssNotExists)?
					}
				}
				DownstreamMessage::SetAttribute { id, name, value } => {
					match value {
						Some(value) => self.get_element(&id)?.attributes.insert(name, value),
						None => self.get_element(&id)?.attributes.remove(&name),
					};
				}
				DownstreamMessage::SetChildren { id, children } => {
					let nodes = self.map_elements(children);
					let old_children = {
						let element = self.get_element(&id)?;

						let old_children = element.children.drain(..).collect::<Vec<_>>();
						element.children = nodes;

						old_children
					};

					self.drop_dependencies(old_children);
				}
				DownstreamMessage::SetRootChildren { children } => {
					let old_roots = self.root_nodes.drain(..).collect::<Vec<_>>();

					self.drop_dependencies(old_roots);
					self.root_nodes = self.map_elements(children);
				}
				DownstreamMessage::SetPagePath { path } => {
					self.path = path;
				}
				DownstreamMessage::SetPageTitle { title } => self.title = Some(title),
				DownstreamMessage::ShowError { message } => self.errors.push(message),
				DownstreamMessage::SetTextContent { id, content } => {
					let element = self.get_element(&id)?;

					element.text_content = Some(content);
				}
			};
		}

		Ok(())
	}

	fn write_nodes_html(&self, nodes: &Vec<TreeNode>, f: &mut fmt::Formatter) -> fmt::Result {
		for node in nodes {
			let (physical, id) = match node {
				TreeNode::Physical(physical) => (physical, None),
				TreeNode::Reference(id) => match self.nodes.get(id) {
					Some(physical) => (physical, Some(id)),
					None => continue,
				},
			};

			if !physical.should_show() {
				continue;
			}

			physical.write_opening_tag(f, id)?;

			if let Some(text) = &physical.text_content {
				write!(f, "{}", text)?;
			}

			self.write_nodes_html(&physical.children, f)?;
			physical.write_closing_tag(f)?;
		}

		Ok(())
	}
}

impl Display for ViewTree {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut head = self.head_tags.join("");
		let lang = &self.lang;

		if let Some(title) = &self.title {
			write!(head, "<title>{title}</title>")?;
		}

		write!(f, "<!DOCTYPE html><html lang=\"{lang}\"><head>{head}")?;

		for (id, css) in &self.css {
			write!(f, "<style id=\"{id}\">{css}</style>")?;
		}

		write!(f, "<head><body>")?;

		self.write_nodes_html(&self.root_nodes, f)?;

		write!(f, "</body></html>")?;

		Ok(())
	}
}
