use crate::ViewId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum UpstreamMessage {
	OnPagePathChanged { new_path: String },
	ActionTrigger { id: ViewId, data: ElementActionPayload },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum DownstreamMessage {
	SetChildren { id: ViewId, children: Vec<GenericElement> },
	SetRootChildren { children: Vec<GenericElement> },
	SetAttribute { id: ViewId, name: String, value: Option<String> },
	AddCssChunk { id: ViewId, content: String },
	DeleteCssChunk { id: ViewId },
	SetTextContent { id: ViewId, content: String },
	SetPagePath { path: String },
	SetPageTitle { title: String },
	ShowError { message: String },
	CancelLoader { action_id: ViewId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenericElement {
	pub id: Option<ViewId>,
	pub tag: String,
	pub actions: Vec<ActionDefinition>,
	pub children: Vec<GenericElement>,
	pub attributes: Vec<(String, String)>,
	pub text_content: Option<String>,
	pub delete_after_ms: Option<u32>,
}

impl GenericElement {
	pub fn new(tag: impl Into<String>) -> GenericElement {
		GenericElement {
			id: None,
			tag: tag.into(),
			actions: Vec::new(),
			children: Vec::new(),
			attributes: Vec::new(),
			text_content: None,
			delete_after_ms: None,
		}
	}

	pub fn action(mut self, action: ActionDefinition) -> GenericElement {
		self.actions.push(action);

		self
	}

	pub fn attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> GenericElement {
		self.attributes.push((name.into(), value.into()));

		self
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActionDefinition {
	pub loader_behavior: ActionLoaderBehavior,
	pub id: ViewId,
	pub event: String,
	pub payload_kind: ElementActionPayloadKind,
}

impl ActionDefinition {
	pub fn new(id: ViewId, event: impl Into<String>) -> ActionDefinition {
		ActionDefinition {
			loader_behavior: ActionLoaderBehavior::Hide,
			id,
			event: event.into(),
			payload_kind: ElementActionPayloadKind::Nothing,
		}
	}

	pub fn loader_behavior(mut self, behavior: ActionLoaderBehavior) -> ActionDefinition {
		self.loader_behavior = behavior;

		self
	}

	pub fn payload_kind(mut self, payload_kind: ElementActionPayloadKind) -> ActionDefinition {
		self.payload_kind = payload_kind;

		self
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ActionLoaderBehavior {
	Show,
	Hide,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ElementActionPayload {
	InputValue { value: String },
	Nothing,
	Switch { value: bool },
	Dynamic { value: Value },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ElementActionPayloadKind {
	InputValue,
	Nothing,
	Switch,
	Dynamic,
}
