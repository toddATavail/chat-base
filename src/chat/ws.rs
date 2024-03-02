use async_openai::types::{
	ChatCompletionRequestMessage, CreateChatCompletionRequestArgs
};
use axum::{
	extract::ws::{Message as WebSocketMessage, WebSocket, WebSocketUpgrade},
	response::IntoResponse
};
use futures::{StreamExt, lock::Mutex, stream::SplitSink};
use std::sync::Arc;
use tracing::{debug, trace};

use super::SessionState;
use super::{AppMessage, Message};
use crate::error_template::AppError;

////////////////////////////////////////////////////////////////////////////////
//                             Websocket support.                             //
////////////////////////////////////////////////////////////////////////////////

/// The handler for the HTTP request, called when the HTTP GET arrives
/// at the start of Websocket negotiation. After this completes, the actual
/// Websocket protocol upgrade occurs. This is the last point
/// where we can extract TCP/IP metadata, HTTP headers, etc.
pub async fn chat_handler(ws: WebSocketUpgrade) -> impl IntoResponse
{
	ws.on_upgrade(move |ws| handle_ws(ws, SessionState::default()))
}

/// The handler for the Websocket connection. This is where we handle the
/// actual Websocket protocol. We receive messages from the client and send
/// messages to the client.
async fn handle_ws(ws: WebSocket, state: SessionState)
{
	let (send, mut recv) = ws.split();
	let send = Arc::new(Mutex::new(send));
	let state = Arc::new(Mutex::new(state));
	while let Some(message) = recv.next().await
	{
		let message = decode_message(message);
		// We have a legitimate message, so process it.
		match message
		{
			None => continue,
			Some(AppMessage::StartChat(messages)) =>
			{
				let send = Arc::clone(&send);
				let state = Arc::clone(&state);
				tokio::spawn(async move {
					let send = Arc::clone(&send);
					chat(messages, &send, &state).await;
				});
			},
			Some(AppMessage::NextChatFragment(_)) =>
			{
				debug!("Received unexpected NextChatFragment message")
			},
			Some(AppMessage::ChatCompleted) =>
			{
				debug!("Received unexpected ChatCompleted message")
			},
			Some(AppMessage::Error(_)) =>
			{
				debug!("Received unexpected Error message")
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
//                                   Chat.                                    //
////////////////////////////////////////////////////////////////////////////////

/// Start a chat with the given messages. This function will send the messages
/// to the OpenAI API and then stream the responses back to the client via a
/// series of [`AppMessage::NextChatFragment`] messages. When the chat is
/// complete, a [`AppMessage::ChatCompleted`] message will be sent.
///
/// If the chat assistant is busy, then a [`AppError::ChatError`] will be
/// returned.
///
/// # Arguments
///
/// - `messages`: The messages to send to the chat assistant.
/// - `send`: The websocket sink to send messages to the client.
/// - `state`: The session state.
async fn chat(
	messages: Vec<Message>,
	send: &Arc<Mutex<SplitSink<WebSocket, WebSocketMessage>>>,
	state: &Arc<Mutex<SessionState>>
)
{
	// If the chat assistant is busy, then we can't start a new chat.
	{
		let mut state = state.lock().await;
		match state.chat_busy
		{
			true =>
			{
				let e = AppError::ChatError;
				debug!("Chat assistant is busy: {e}");
				return
			},
			false =>
			{
				trace!("Chat assistant is now busy");
				state.chat_busy = true;
			}
		}
	}
	// Deal with the chat and present the conclusion to the client.
	match just_chat(messages, send, state).await
	{
		Ok(_) =>
		{
			trace!("Chat completed");
			let _ = AppMessage::ChatCompleted.send_to_client(send).await;
		},
		Err(e) =>
		{
			debug!("Chat error: {:?}", e);
			let _ = AppMessage::Error(e).send_to_client(send).await;
		}
	};
	// The chat assistant is no longer busy.
	let mut state = state.lock().await;
	state.chat_busy = false;
	trace!("Chat assistant is now available");
}

/// Start a chat with the given messages. This function will send the messages
/// to the OpenAI API and then stream the responses back to the client via a
/// series of [`AppMessage::NextChatFragment`] messages.
///
/// This function does not handle the chat assistant's busy state. The caller
/// must handle this, and ensure that the state is always instantaneously
/// correct.
///
/// # Arguments
///
/// - `messages`: The messages to send to the chat assistant.
/// - `send`: The websocket sink to send messages to the client.
/// - `state`: The session state.
///
/// # Returns
///
/// This function returns `()` if the chat completes successfully. If there is
/// an error, then an [`AppError`] is returned. In neither case is the
/// conclusion transmitted to the client. This is the responsibility of the
/// caller.
async fn just_chat(
	messages: Vec<Message>,
	send: &Arc<Mutex<SplitSink<WebSocket, WebSocketMessage>>>,
	state: &Arc<Mutex<SessionState>>
) -> Result<(), AppError>
{
	// Convert the messages to the OpenAI message type.
	let messages: Vec<ChatCompletionRequestMessage> = messages
		.iter()
		.map(TryInto::try_into)
		.collect::<Result<Vec<_>, _>>()
		.map_err(|_| AppError::ChatError)?;
	// Create a chat stream.
	let request = CreateChatCompletionRequestArgs::default()
		.model(MODEL)
		.messages(messages)
		.max_tokens(150u16)
		.temperature(0.8f32)
		.top_p(0.95f32)
		.stream(true)
		.build()
		.map_err(|_| AppError::ChatError)?;
	let mut chat_stream = {
		let client = &state.lock().await.chat_client;
		client
			.chat()
			.create_stream(request)
			.await
			.map_err(|_| AppError::ChatError)?
	};
	// Process the chat stream.
	while let Some(fragment) = chat_stream.next().await
	{
		trace!("Received chat fragment: {:#?}", fragment);
		let mut fragment = fragment.map_err(|_| AppError::ChatError)?;
		let choice = fragment.choices.first_mut().ok_or(AppError::ChatError)?;
		let fragment = match choice.finish_reason
		{
			Some(reason) =>
			{
				trace!("Chat finished: {:#?}", reason);
				break
			},
			None => choice.delta.content.take().ok_or(AppError::ChatError)?
		};
		let message = AppMessage::NextChatFragment(fragment);
		message.send_to_client(send).await?;
	}
	Ok(())
}

////////////////////////////////////////////////////////////////////////////////
//                            Decoding utilities.                             //
////////////////////////////////////////////////////////////////////////////////

/// Decode an [`AppMessage`] from the given [`WebSocketMessage`]. Answer
/// `None` if a valid message could not be decoded.
fn decode_message(
	message: Result<WebSocketMessage, axum::Error>
) -> Option<AppMessage>
{
	// Extract the raw message from the websocket.
	let message = match message
	{
		Ok(message) => message,
		Err(e) =>
		{
			debug!("Error receiving message: {}", e);
			return None
		}
	};
	// Only handle binary message frames.
	let message = match message
	{
		WebSocketMessage::Binary(message) =>
		{
			trace!("Received frame: Binary: {:#?}", message);
			message
		},
		WebSocketMessage::Text(message) =>
		{
			trace!("Received unsupported frame: Text: {:#?}", message);
			return None
		},
		WebSocketMessage::Close(close) =>
		{
			trace!("Received frame: Close: {:#?}", close);
			return None
		},
		WebSocketMessage::Ping(ping) =>
		{
			trace!("Received unsupported frame: Ping: {:#?}", ping);
			return None
		},
		WebSocketMessage::Pong(pong) =>
		{
			trace!("Received unsupported frame: Pong: {:#?}", pong);
			return None
		}
	};
	// Use bincode to deserialize the message.
	match bincode::deserialize(&message)
	{
		Ok(message) =>
		{
			trace!("Deserialized message: {:#?}", message);
			Some(message)
		},
		Err(e) =>
		{
			debug!("Error deserializing message: {}", e);
			None
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
//                                 Constants.                                 //
////////////////////////////////////////////////////////////////////////////////

/// The model to use for the chat completion. This is the model that will be
/// used to generate the chat responses.
const MODEL: &str = "mistralai_mixtral-8x7b-instruct-v0.1";
