use crate::{
	state::{DownstreamMessage, ElementActionPayload, GenericElement, UpstreamMessage},
	ViewId,
};
use futures_util::{poll, FutureExt};
use log::error;
use rutils::{rpc, RpcCaller, RpcHandler};
use std::{collections::HashMap, fmt::Debug, task::Poll};
use tokio::sync::mpsc::{channel, Receiver, Sender};

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

pub struct Context {
	actions: HashMap<ViewId, ElementActionPayload>,
	path_change: Option<String>,
	defined_children: Vec<GenericElement>,
	queued_messages: Vec<DownstreamMessage>,
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

	pub fn reduce_children(&mut self, new_tag: impl Into<String>) -> GenericElement {
		let mut element = GenericElement::new(new_tag);
		element.children = self.defined_children.drain(..).collect();

		element
	}

	pub fn push_child(&mut self, child: GenericElement) {
		self.defined_children.push(child)
	}

	pub fn set_page_title(&mut self, new_title: impl Into<String>) {
		self.queued_messages.push(DownstreamMessage::SetPageTitle { title: new_title.into() })
	}

	pub fn get_path_change(&self) -> Option<&str> {
		self.path_change.as_deref()
	}
}

pub trait View: Debug {
	fn update(&mut self, context: &mut Context) -> ViewResult<()>;
}

pub struct ViewDriver<T: View> {
	root: T,
	initial_path: String,
	rpc_handler: RpcHandler<(), ViewDriverSubscription>,
}

impl<T: View> ViewDriver<T> {
	pub async fn drive(mut self) {
		let mut context = Context {
			actions: HashMap::new(),
			defined_children: Vec::new(),
			path_change: Some(self.initial_path),
			queued_messages: Vec::new(),
		};

		fn handle_error(context: &mut Context, err: NodeError) {
			match err {
				NodeError::Internal(message) => {
					error!("An internal error occurred in a view. {}", message);

					context.queued_messages.push(DownstreamMessage::ShowError {
						message: "An internal error occurred".into(),
					});
				}
				NodeError::User(message) => context.queued_messages.push(DownstreamMessage::ShowError { message }),
			};
		}

		fn drive_root<T: View>(context: &mut Context, root: &mut T) -> ViewResult<()> {
			let pre_update_actions = context.actions.len();

			root.update(context)?;

			if !context.actions.is_empty() {
				if context.actions.len() == pre_update_actions {
					Err(NodeError::internal(format!("entire event loop iteration didn't consume any actions")))?
				}
				drive_root(context, root)?
			}

			Ok(())
		}

		if let Err(err) = drive_root(&mut context, &mut self.root) {
			handle_error(&mut context, err)
		}

		if !context.defined_children.is_empty() {
			context.queued_messages.push(DownstreamMessage::SetRootChildren {
				children: context.defined_children.drain(..).collect(),
			})
		}

		loop {
			let request = match self.rpc_handler.next().await {
				Some(req) => req,
				None => break,
			};

			let (downstream_sender, downstream_receiver) = channel(1000);
			let (upstream_sender, mut upstream_receiver) = channel(1000);

			request.responder.respond(ViewDriverSubscription {
				downstream_receiver,
				upstream_sender,
			});

			loop {
				let downstream_messages = context.queued_messages.drain(..).collect::<Vec<_>>();
				if !downstream_messages.is_empty() {
					// if we fail to send, that is probably because the handle was dropped, we add the message back to the queue so that they will be sent
					// downstream on the handle
					//
					// We don't need to break here because the break should happen at the next .recv call
					if let Err(mut err) = downstream_sender.send(downstream_messages).await {
						context.queued_messages.append(&mut err.0);
					}
				}

				let messages = match upstream_receiver.recv().await {
					Some(messages) => messages,
					None => break,
				};

				context.path_change = None;

				for message in messages {
					if let UpstreamMessage::OnPagePathChanged { new_path } = message {
						context.path_change = Some(new_path);
						continue;
					}

					if let UpstreamMessage::ActionTrigger { id, data } = message {
						context.actions.insert(id, data);
						continue;
					}

					handle_error(&mut context, NodeError::user("received unknown message"));
				}

				if let Err(err) = drive_root(&mut context, &mut self.root) {
					handle_error(&mut context, err)
				}
			}
		}
	}
}

pub struct ViewDriverHandle {
	rpc_caller: RpcCaller<(), ViewDriverSubscription>,
}

impl ViewDriverHandle {
	pub async fn subscribe(&self) -> Option<ViewDriverSubscription> {
		self.rpc_caller.call(()).await
	}
}

pub fn create_driver<T: View + Debug>(root: T, path: impl Into<String>) -> (ViewDriverHandle, ViewDriver<T>) {
	let (rpc_caller, rpc_handler) = rpc(1000);

	let driver = ViewDriver {
		root,
		initial_path: path.into(),
		rpc_handler,
	};

	let handle = ViewDriverHandle { rpc_caller };

	(handle, driver)
}

pub struct ViewDriverSubscription {
	upstream_sender: Sender<Vec<UpstreamMessage>>,
	downstream_receiver: Receiver<Vec<DownstreamMessage>>,
}

impl ViewDriverSubscription {
	/// Take some messages to be sent downstream. Returns `None` if the `ViewDriver` subscribed to is dropped.
	pub async fn take_next_downstream_messages(&mut self) -> Option<Vec<DownstreamMessage>> {
		self.downstream_receiver.recv().await
	}

	/// Takes the next series messages that is currently available without waiting.
	pub async fn take_current_downstream_messages(&mut self) -> Option<Vec<DownstreamMessage>> {
		match poll!(self.downstream_receiver.recv().boxed()) {
			Poll::Ready(inner) => Some(inner),
			Poll::Pending => None,
		}
		.flatten()
	}

	/// Takes all messages that are current available without waiting.
	pub async fn take_many_current_downstream_messages(&mut self) -> Option<Vec<DownstreamMessage>> {
		let mut messages: Option<Vec<DownstreamMessage>> = None;

		loop {
			let mut new_messages = match self.take_current_downstream_messages().await {
				Some(messages) => messages,
				None => break,
			};

			if let Some(messages) = &mut messages {
				messages.append(&mut new_messages);
			} else {
				messages = Some(new_messages)
			}
		}

		messages
	}

	/// Try to send `messages` upstream, removing them the vector. If the send fails, no messages will be removed from the vector.
	pub async fn send_upstream_messages(&self, messages: &mut Vec<UpstreamMessage>) {
		match self.upstream_sender.send(messages.drain(..).collect()).await {
			Err(mut err) => messages.append(&mut err.0),
			_ => (),
		};
	}
}
