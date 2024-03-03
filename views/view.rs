use error_stack::{Report, ResultExt};
use log::error;
use rand::random;
use rutils::errors;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
	collections::HashMap,
	fmt::{self, Debug, Display},
	str::FromStr,
};

errors!(FailedToDecodeViewId, ViewIdBadLength);

pub enum NodeError {
	User(String),
	Internal(String),
}

pub type ViewResult<T> = std::prelude::v1::Result<T, NodeError>;

impl NodeError {
	pub fn user(message: impl Into<String>) -> NodeError {
		NodeError::User(message.into())
	}

	pub fn internal(message: impl Into<String>) -> NodeError {
		NodeError::Internal(message.into())
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ViewId([u8; 8]);

impl ViewId {
	pub fn generate() -> ViewId {
		ViewId(random())
	}
}

impl Display for ViewId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&hex::encode(self.0))
	}
}

impl FromStr for ViewId {
	type Err = Report<Error>;

	fn from_str(value: &str) -> Result<ViewId> {
		let bytes = hex::decode(value).change_context(Error::FailedToDecodeViewId)?;
		if bytes.len() != 8 {
			Err(Error::ViewIdBadLength)?
		}

		Ok(ViewId(bytes.as_slice().try_into().unwrap()))
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Message {
	SetChildren { id: String, children: Vec<GenericElement> },
	SetClassList { id: String, class_list: String },
	AddCssChunk { id: String, content: String },
	DeleteCssChunk { id: String, content: String },
	SetPageHash { hash: String },
	OnHashChanged { new_hash: String },
	SetPageTitle { title: String },
	ActionTrigger { id: String, data: ElementActionPayload },
	Error { message: String },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenericElement {
	id: Option<String>,
	actions: Vec<ActionDefinition>,
	children: Vec<GenericElement>,
	class_list: String,
	delete_after_ms: Option<u32>,
}

impl GenericElement {
	pub fn new() -> GenericElement {
		GenericElement {
			id: None,
			actions: Vec::new(),
			children: Vec::new(),
			class_list: String::new(),
			delete_after_ms: None,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActionDefinition {
	show_loader: bool,
	id: String,
	data: ElementActionPayloadKind,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ElementActionPayload {
	InputValue { value: String },
	Nothing,
	Switch { value: bool },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ElementActionPayloadKind {
	InputValue,
	Nothing,
	Switch,
}

pub struct Context {
	actions: HashMap<ViewId, ElementActionPayload>,
	path_change: Option<String>,
	defined_children: Vec<GenericElement>,
	queued_messages: Vec<Message>,
}

impl Context {
	pub fn take_action(&mut self, id: &ViewId) -> Option<ElementActionPayload> {
		self.actions.remove(id)
	}

	pub fn deny_children(&self) -> ViewResult<()> {
		if !self.defined_children.is_empty() {
			Err(NodeError::internal("Children were defined, but element is not prepared to use them"))?
		}

		Ok(())
	}

	pub fn reduce_children(&mut self) -> GenericElement {
		let mut element = GenericElement::new();
		element.children = self.defined_children.drain(..).collect();

		element
	}

	pub fn push_child(&mut self, child: GenericElement) {
		self.defined_children.push(child)
	}

	pub fn set_page_title(&mut self, new_title: impl Into<String>) {
		self.queued_messages.push(Message::SetPageTitle { title: new_title.into() })
	}

	pub fn get_path_change(&self) -> Option<&str> {
		self.path_change.as_deref()
	}
}

pub trait View: Debug {
	fn update(&mut self, context: &mut Context) -> ViewResult<()>;
}

pub struct ViewDriver<T: View> {
	context: Context,
	root: T,
}

impl<T: View> ViewDriver<T> {
	pub fn new(mut root: T, path: String) -> Result<ViewDriver<T>> {
		let mut context = Context {
			actions: HashMap::new(),
			defined_children: Vec::new(),
			path_change: Some(path),
			queued_messages: Vec::new(),
		};

		match root.update(&mut context) {
			Ok(_) => (),
			Err(NodeError::Internal(message)) => {
				error!("An internal error occurred in a view. {}", message);

				context.queued_messages.push(Message::Error {
					message: "An internal error occurred".into(),
				});
			}
			Err(NodeError::User(message)) => context.queued_messages.push(Message::Error { message }),
		}

		context.queued_messages.push(Message::SetChildren {
			id: "root".into(),
			children: context.defined_children.drain(..).collect(),
		});

		Ok(ViewDriver { context, root })
	}

	pub fn provide_upstream_events(&mut self, events: &str) -> ViewResult<()> {
		self.context.path_change = None;

		let messages = from_str::<Vec<Message>>(events).map_err(|_| NodeError::user("failed to deserialize request events"))?;

		for message in messages {
			if let Message::OnHashChanged { new_hash } = message {
				self.context.path_change = Some(new_hash);
				continue;
			}

			if let Message::ActionTrigger { id, data } = message {
				self.context
					.actions
					.insert(id.parse().map_err(|_| NodeError::user("failed to parse view id"))?, data);
				continue;
			}

			Err(NodeError::user("received unknown message"))?
		}

		self.drive_root();

		Ok(())
	}

	pub fn take_downstream_events(&mut self) -> ViewResult<String> {
		to_string(&self.context.queued_messages.drain(..).collect::<Vec<_>>()).map_err(|_| NodeError::internal("failed to serialize messages for client"))
	}

	fn drive_root(&mut self) -> ViewResult<()> {
		let pre_update_actions = self.context.actions.len();

		self.root.update(&mut self.context)?;

		if !self.context.actions.is_empty() {
			if self.context.actions.len() == pre_update_actions {
				Err(NodeError::internal("entire event loop iteration didn't consume any actions"))?
			}
			self.drive_root()?
		}

		Ok(())
	}
}
