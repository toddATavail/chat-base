#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use async_openai::{
	Client,
	config::OpenAIConfig,
	error::OpenAIError,
	types::{
		ChatCompletionRequestAssistantMessageArgs,
		ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
		ChatCompletionRequestUserMessageArgs, CreateChatCompletionResponse
	}
};

#[cfg(feature = "ssr")]
use axum::extract::ws::{Message as WebSocketMessage, WebSocket};
#[cfg(feature = "ssr")]
use futures::{SinkExt, lock::Mutex, stream::SplitSink};
#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;
#[cfg(feature = "ssr")]
use leptos::server_fn::error::NoCustomError;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use tracing::{debug, trace};

use crate::error_template::AppError;

////////////////////////////////////////////////////////////////////////////////
//                       Application message protocol.                        //
////////////////////////////////////////////////////////////////////////////////

/// The application messages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppMessage
{
	/// A chat completion request, sent by the client.
	StartChat(Vec<Message>),

	/// A chat fragment reply, sent by the server in response to a
	/// [`StartChat`](Self::StartChat) message.
	NextChatFragment(String),

	/// A chat conclusion reply, sent by the server in response to a
	/// [`StartChat`](Self::StartChat) message.
	ChatCompleted,

	/// An error reply, sent by the server.
	Error(AppError)
}

impl AppMessage
{
	/// Send the message to the client. Handles serialization, framing, sending,
	/// and logging.
	#[cfg(feature = "ssr")]
	pub async fn send_to_client(
		&self,
		send: &Arc<Mutex<SplitSink<WebSocket, WebSocketMessage>>>
	) -> Result<(), AppError>
	{
		trace!("Sending message: {:?}", self);
		let bytes =
			bincode::serialize(self).map_err(|_| AppError::ServerError)?;
		let message = WebSocketMessage::Binary(bytes);
		match send
			.lock()
			.await
			.send(message)
			.await
			.map_err(|_| AppError::ServerError)
		{
			Ok(_) =>
			{
				trace!("Sent message: {:?}", self);
				Ok(())
			},
			Err(e) =>
			{
				debug!("Error sending message: {}", e);
				Err(AppError::ServerError)
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
//                              Session support.                              //
////////////////////////////////////////////////////////////////////////////////

/// Each websocket connection has a context that holds session data.
#[cfg(feature = "ssr")]
#[derive(Debug)]
pub(super) struct SessionState
{
	/// The chat client.
	pub chat_client: Client<OpenAIConfig>,

	/// Whether the chat assistant is currently busy.
	pub chat_busy: bool
}

#[cfg(feature = "ssr")]
impl Default for SessionState
{
	fn default() -> Self
	{
		Self {
			chat_client: Client::with_config(
				OpenAIConfig::new()
					.with_api_base(get_base_url())
					.with_api_key(get_key())
			),
			chat_busy: false
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
//                                Chat types.                                 //
////////////////////////////////////////////////////////////////////////////////

/// The role of a message in the chat.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role
{
	/// The assistant's role. This corresponds to the assistant that generates
	/// the chat responses.
	Assistant,

	/// The system's role. This corresponds to the system message that
	/// constrains how the assistant generates the chat responses.
	System,

	/// The user's role. This corresponds to user who is interacting with the
	/// chat.
	User
}

impl Role
{
	/// Create a message with no content, using the receiver as the role.
	pub fn empty(self) -> Message { self.message(String::new()) }

	/// Create a message with the given content, using the receiver as the role.
	pub fn message(self, content: String) -> Message
	{
		Message {
			role: self,
			content
		}
	}
}

#[cfg(feature = "ssr")]
impl From<async_openai::types::Role> for Role
{
	fn from(role: async_openai::types::Role) -> Self
	{
		match role
		{
			async_openai::types::Role::Assistant => Role::Assistant,
			async_openai::types::Role::System => Role::System,
			async_openai::types::Role::User => Role::User,
			_ => unreachable!()
		}
	}
}

/// A message in the chat, either a system prompt or a message that is sent
/// between the user and the assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message
{
	/// The role of the message in the chat.
	pub role: Role,

	/// The content of the message.
	pub content: String
}

impl Message
{
	/// Get the role of the message.
	#[inline]
	pub fn role(&self) -> Role { self.role }

	/// Get the content of the message.
	#[inline]
	pub fn content(&self) -> &str { &self.content }
}

#[cfg(feature = "ssr")]
impl TryFrom<&CreateChatCompletionResponse> for Message
{
	type Error = ServerFnError;

	fn try_from(
		response: &CreateChatCompletionResponse
	) -> Result<Self, Self::Error>
	{
		let message = response
			.choices
			.first()
			.ok_or_else(|| {
				ServerFnError::<NoCustomError>::Response(
					"response contained no options".to_string()
				)
			})?
			.message
			.clone();
		let role = message.role.into();
		let content = message.content.unwrap_or_else(String::new).to_string();
		Ok(Self { role, content })
	}
}

#[cfg(feature = "ssr")]
impl TryFrom<&Message> for ChatCompletionRequestMessage
{
	type Error = OpenAIError;

	fn try_from(message: &Message) -> Result<Self, Self::Error>
	{
		match message.role
		{
			Role::Assistant =>
			{
				Ok(ChatCompletionRequestAssistantMessageArgs::default()
					.content(message.content.clone())
					.build()?
					.into())
			},
			Role::System =>
			{
				Ok(ChatCompletionRequestSystemMessageArgs::default()
					.content(message.content.clone())
					.build()?
					.into())
			},
			Role::User => Ok(ChatCompletionRequestUserMessageArgs::default()
				.content(message.content.clone())
				.build()?
				.into())
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
//                               Configuration.                               //
////////////////////////////////////////////////////////////////////////////////

/// Get the base URL for the OpenAI API. This is where the API is hosted.
#[cfg(feature = "ssr")]
fn get_base_url() -> String
{
	std::env::var("OPENAI_API_URL").unwrap_or_else(|_| URL.to_string())
}

/// Get the API key for the OpenAI API. This is used to authenticate the user
/// with the API.
#[cfg(feature = "ssr")]
fn get_key() -> String
{
	std::env::var("OPENAI_TOKEN").unwrap_or_else(|_| KEY.to_string())
}

////////////////////////////////////////////////////////////////////////////////
//                                 Constants.                                 //
////////////////////////////////////////////////////////////////////////////////

/// The URL for the OpenAI API. This is where LM Studio is hosting the model.
#[cfg(feature = "ssr")]
const URL: &str = "http://localhost:5115/v1";

/// The API key for the OpenAI API. Theoretically, this is used to authenticate
/// the user with the API. But since we're using a local server, it's not
/// actually needed.
#[cfg(feature = "ssr")]
const KEY: &str = "not-needed";
