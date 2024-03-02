use codee::binary::BincodeSerdeCodec;
use leptos::{html, prelude::*, server::LocalResource};
use leptos_use::{
	UseClipboardReturn, UseWebSocketReturn, core::ConnectionReadyState,
	use_clipboard, use_websocket
};
use log::{debug, trace};
use std::time::Duration;
use uuid::Uuid;

use crate::chat::{AppMessage, Message, Role};

use super::icons::{
	AssistantImage, CopiedImage, CopyImage, DeleteImage, EditImage,
	RegenerateImage, RewindImage, SystemImage, UserImage
};

////////////////////////////////////////////////////////////////////////////////
//                              Chat utilities.                               //
////////////////////////////////////////////////////////////////////////////////

/// Find the index of the item with the specified identifier.
fn to_index<I, T>(id: Uuid, iter: &I) -> Option<usize>
where
	for<'i> &'i I: IntoIterator<Item = &'i (Uuid, T)>
{
	iter.into_iter()
		.enumerate()
		.find(|(_, (i, _))| *i == id)
		.map(|(index, _)| index)
}

////////////////////////////////////////////////////////////////////////////////
//                              Chat components.                              //
////////////////////////////////////////////////////////////////////////////////

/// A complete interactive chat with an AI assistant.
#[component]
pub fn Chat() -> impl IntoView
{
	// The system message. We need to use a local resource in order to read this
	// signal in an effect.
	let system_message =
		LocalResource::new(|| async move { system_message(None).await });
	// An invisible component that we can scroll into view to ensure that the
	// last message, even if incomplete, is always at the bottom.
	let bottom = NodeRef::<html::Div>::new();
	// The messages.
	let (messages, set_messages) = signal(Vec::<(Uuid, Message)>::new());
	Effect::new(move |_| {
		if let Some(message) = system_message.get()
		{
			if let Ok(message) = &*message
			{
				set_messages.update(|messages| {
					messages.push((Uuid::new_v4(), message.clone()));
				});
			}
		}
	});
	// The user's latest incomplete message.
	let (user_message, set_user_message) = signal(String::new());
	// The assistant's latest incomplete message.
	let (assistant_message, set_assistant_message) = signal(String::new());
	// Which message the user is editing, is any.
	let (editing, set_editing) = signal(None::<Uuid>);
	// Whether the assistant is busy generating the next message.
	let (pending, set_pending) = signal(false);
	// Whether to disable message-specific actions in the user interface.
	let disabled = pending;
	let UseWebSocketReturn {
		// The WebSocket connection state.
		ready_state,
		// How to read the latest binary message from the assistant.
		message,
		// How to send the next binary message to the assistant.
		send,
		..
	} = use_websocket::<AppMessage, AppMessage, BincodeSerdeCodec>("/api/chat");
	// Whether we can send a message to the assistant.
	let can_send =
		move || ready_state() == ConnectionReadyState::Open && !pending();
	// How to obtain the next message from the assistant. Accepts the
	// complete message history, which must already contain the user's latest
	// message.
	let chat = move |messages: &Vec<(Uuid, Message)>| {
		set_pending(true);
		let messages = messages
			.iter()
			.map(|(_, message)| message.clone())
			.collect::<Vec<_>>();
		trace!("Sending messages: {:#?}", messages);
		send(&AppMessage::StartChat(messages));
	};
	// How to update the history with the next message from the assistant. Also
	// scrolls the history to the bottom.
	Effect::new(move |_| {
		// Read the next message from the websocket.
		if let Some(message) = message()
		{
			match message
			{
				// Update the assistant's latest incomplete message with the
				// new fragment.
				AppMessage::NextChatFragment(fragment) =>
				{
					trace!("Received fragment: {fragment}");
					set_assistant_message.update(|m| {
						m.push_str(&fragment);
					});
					let bottom = bottom.get().unwrap();
					bottom.scroll_into_view_with_bool(false);
				},
				// The chat completion is done. Clear `pending` and update
				// the history with the completed message. Scroll the
				// history to the bottom.
				AppMessage::ChatCompleted =>
				{
					trace!("Chat completed");
					set_pending(false);
					let complete = set_assistant_message
						.try_update(|message| {
							let trimmed = message.trim();
							let complete =
								Role::Assistant.message(trimmed.into());
							message.clear();
							complete
						})
						.unwrap();
					// Sometimes the assistant declines to create more
					// content. This is fine, but we don't want to add an
					// empty message to the history.
					if !complete.content.is_empty()
					{
						set_messages.update(move |messages| {
							messages.push((Uuid::new_v4(), complete));
							trace!("History: {messages:#?}");
						});
					}
					let bottom = bottom.get().unwrap();
					bottom.scroll_into_view_with_bool(false);
				},
				unexpected =>
				{
					debug!("Unexpected message: {:?}", unexpected);
				}
			}
		}
	});
	// How to identify the last message in the history.
	let last_message = move || messages().last().map(|(id, _)| *id);
	// How to regenerate am assistant message.
	let regenerate = {
		let chat = chat.clone();
		move |id| {
			Signal::derive(move || {
				if Some(id) == last_message()
				{
					let chat = chat.clone();
					Some(move |id| {
						set_messages.update(|messages| {
							let index = to_index(id, messages).unwrap();
							messages.remove(index);
						});
						chat(&messages());
					})
				}
				else
				{
					None
				}
			})
		}
	};
	// How to rewind the conversation to the specified message.
	let rewind = move |id| {
		Signal::derive(move || {
			if Some(id) == last_message()
			{
				None
			}
			else
			{
				Some(move |id| {
					set_messages.update(|messages| {
						let index = to_index(id, messages).unwrap();
						messages.truncate(index + 1);
					});
				})
			}
		})
	};

	view! {
		<div class="h-screen flex flex-col">
			<div class="overflow-y-auto flex-grow">
				<Transition fallback=move || view! {
					<div class="mx-auto h-64 w-2/3">
						<div class="skeleton h-full w-full"></div>
					</div>
				}>
					{move || {
						let _ = system_message.get();
					}}
				</Transition>
				<For
					each=messages
					key=move |(id, _)| *id
					children={
						move |(id, message)| view! {
							<ChatMessage
								id=id
								message=message
								disabled=disabled
								editing=editing
								set_editing=set_editing
								edit=move |id, content| {
									set_messages.update(|messages| {
										let index = to_index(id, messages).unwrap();
										let (id, message) = &mut messages[index];
										*id = Uuid::new_v4();
										message.content = content;
									});
								}
								regenerate={regenerate.clone()(id)}
								rewind={rewind(id)}
								delete=move |id| {
									set_messages.update(|messages| {
										let index = to_index(id, messages)
											.unwrap();
										messages.remove(index);
									});
								}
							/>
						}
					}
				/>
				<Show when=pending>
					<IncompleteAssistantMessage message=assistant_message />
				</Show>
				<div node_ref=bottom class="h-4"></div>
			</div>
			<div class="flex-none mt-4 mb-8">
				<form on:submit={
					let chat = chat.clone();
					move |ev| {
						// Do not actually submit the form.
						ev.prevent_default();
						// Only send a message if the assistant is free.
						if can_send()
						{
							let message = user_message();
							let trimmed = message.trim();
							let trimmed =
								if trimmed.is_empty() { None }
								else { Some(trimmed.to_string()) };
							// Only update the history if the user has entered a
							// message.
							if trimmed.is_some()
							{
								// Add the untrimmed message to the history.
								set_messages.update(|messages| {
									messages.push((
										Uuid::new_v4(),
										Role::User.message(message)
									))
								});
								set_user_message(String::new());
							}
							// Allow the assistant to generate the next message,
							// even if the user didn't enter a message.
							chat(&messages());
							// Scroll the history to the bottom.
							let bottom = bottom.get().unwrap();
							bottom.scroll_into_view_with_bool(false);
					}
					}
				}>
					<Transition fallback=move || view! {
						<div class="mx-auto h-8 w-5/6">
							<div class="skeleton h-full w-full"></div>
						</div>
					}>
					{
						// We don't need the system message, but we do want to
						// ghost the input while the system message is loading.
						let _ = system_message.get();
						view! {
							<div class="flex justify-center">
								<input
									id="user_message"
									type="text"
									placeholder="Type a message…"
									on:input=move |ev| {
										set_user_message(event_target_value(&ev))
									}
									prop:value=user_message
									class="w-5/6"
									autofocus
								/>
							</div>
						}
					}
					</Transition>
					<input type="submit" hidden/>
				</form>
			</div>
		</div>
	}
}

/// Represents the main chat component used to render a chat message.
///
/// # Arguments
///
/// * `id` - Specifies the message identifier.
/// * `message` - Specifies the content of the message.
/// * `disabled` - Indicates whether the message-specific actions should be
///   disabled.
/// * `editing` - Indicates which message is being edited, if any.
/// * `set_editing` - Updates the `editing` indicator.
/// * `edit` - Updates the content of the message.
/// * `regenerate` - Enables the user to regenerate the message. This is
///   available for assistant messages only.
/// * `rewind` - Enables the user to rewind the conversation to the specified
///   message.
/// * `delete` - Enables the user to delete the message.
#[component]
pub fn ChatMessage<D, E, F, R, X>(
	id: Uuid,
	message: Message,
	disabled: D,
	editing: ReadSignal<Option<Uuid>>,
	set_editing: WriteSignal<Option<Uuid>>,
	edit: E,
	regenerate: Signal<Option<F>>,
	rewind: Signal<Option<R>>,
	delete: X
) -> impl IntoView
where
	D: Fn() -> bool + Send + Sync + Clone + 'static,
	E: FnMut(Uuid, String) + 'static,
	F: FnMut(Uuid) + Clone + Send + Sync + 'static,
	R: FnMut(Uuid) + Clone + Send + Sync + 'static,
	X: FnMut(Uuid) + 'static
{
	match message.role
	{
		Role::Assistant => view! {
			<AssistantMessage
				id=id
				message=message
				disabled=disabled
				editing=editing
				set_editing=set_editing
				edit=edit
				regenerate=regenerate
				rewind=rewind
				delete=delete
			/>
		}
		.into_any(),
		Role::System => view! { <SystemMessage message=message/> }.into_any(),
		Role::User => view! {
			<UserMessage
				id=id
				message=message
				editing=editing
				set_editing=set_editing
				edit=edit
				disabled=disabled
				rewind=rewind
				delete=delete
			/>
		}
		.into_any()
	}
}

/// A system message.
#[component]
pub fn SystemMessage(message: Message) -> impl IntoView
{
	view! {
		<div class="flex justify-center">
			<div class="card w-2/3 bg-slate-500 text-black xl-shadow">
				<figure><SystemImage /></figure>
				<div class="card-body text-xs whitespace-pre font-mono">
					<p>{message.content}</p>
				</div>
			</div>
		</div>
	}
}

/// Represents a user message.
///
/// # Arguments
///
/// * `id` - Specifies the message identifier.
/// * `message` - Specifies the content of the message.
/// * `disabled` - Indicates whether the message-specific actions should be
///   disabled.
/// * `editing` - Indicates which message is being edited, if any.
/// * `set_editing` - Updates the `editing` indicator.
/// * `edit` - Updates the content of the message.
/// * `rewind` - Enables the user to rewind the conversation to the specified
///   message.
/// * `delete` - Enables the user to delete the message.
#[component]
pub fn UserMessage<D, E, R, X>(
	id: Uuid,
	message: Message,
	disabled: D,
	editing: ReadSignal<Option<Uuid>>,
	set_editing: WriteSignal<Option<Uuid>>,
	edit: E,
	rewind: Signal<Option<R>>,
	delete: X
) -> impl IntoView
where
	D: Fn() -> bool + Clone + Send + Sync + 'static,
	E: FnMut(Uuid, String) + 'static,
	R: FnMut(Uuid) + Clone + Send + Sync + 'static,
	X: FnMut(Uuid) + 'static
{
	// This is a workaround for shortcomings in the `view!` macro and the type
	// deduction algorithm. The `regenerate` closure is not allowed to be `None`
	// in the `view!` macro, so we have to create a dummy `regenerate` closure
	// that answers `None` instead.
	#[allow(unused_mut, unused_assignments)]
	let mut regenerate = Some(|_| ());
	regenerate = None;
	let regenerate = Signal::derive(move || regenerate);
	view! {
		<ChatBubble
			id=id
			message=message
			chat_class={ "chat-end mr-8".to_string() }
			portrait={ view! { <UserImage/> } }
			bubble_color={ "bg-sky-300".to_string() }
			disabled=disabled
			editing=editing
			set_editing=set_editing
			edit=edit
			regenerate=regenerate
			rewind=rewind
			delete=delete
		/>
	}
}

/// Represents an assistant message.
///
/// # Arguments
///
/// * `id` - Specifies the message identifier.
/// * `message` - Specifies the content of the message.
/// * `disabled` - Indicates whether the message-specific actions should be
///   disabled.
/// * `editing` - Indicates which message is being edited, if any.
/// * `set_editing` - Updates the `editing` indicator.
/// * `edit` - Updates the content of the message.
/// * `regenerate` - Enables the user to regenerate the message.
/// * `rewind` - Enables the user to rewind the conversation to the specified
///   message.
/// * `delete` - Enables the user to delete the message.
#[component]
pub fn AssistantMessage<D, E, F, R, X>(
	id: Uuid,
	message: Message,
	disabled: D,
	editing: ReadSignal<Option<Uuid>>,
	set_editing: WriteSignal<Option<Uuid>>,
	edit: E,
	regenerate: Signal<Option<F>>,
	rewind: Signal<Option<R>>,
	delete: X
) -> impl IntoView
where
	D: Fn() -> bool + Clone + Send + Sync + 'static,
	E: FnMut(Uuid, String) + 'static,
	F: FnMut(Uuid) + Clone + Send + Sync + 'static,
	R: FnMut(Uuid) + Clone + Send + Sync + 'static,
	X: FnMut(Uuid) + 'static
{
	view! {
		<ChatBubble
			id=id
			message=message
			chat_class={ "chat-start ml-8".to_string() }
			portrait={ view! { <AssistantImage/> } }
			bubble_color={ "bg-green-300".to_string() }
			disabled=disabled
			editing=editing
			set_editing=set_editing
			edit=edit
			regenerate=regenerate
			rewind=rewind
			delete=delete
		/>
	}
}

/// Represents a chat bubble.
///
/// # Arguments
///
/// * `id` - The message identifier.
/// * `message` - The content of the message.
/// * `chat_class` - The class to apply to the chat bubble.
/// * `portrait` - The portrait to display with the message.
/// * `bubble_color` - The color of the chat bubble.
/// * `disabled` - A boolean indicating whether the message-specific actions
///   should be disabled.
/// * `editing` - A boolean indicating which message is being edited, if any.
/// * `set_editing` - A function that updates the `editing` indicator.
/// * `edit` - A function that updates the content of the message.
/// * `regenerate` - A function that enables the user to regenerate the message.
/// * `rewind` - A function that enables the user to rewind the conversation to
///   the message.
/// * `delete` - A function that enables the user to delete the message.
#[component]
pub fn ChatBubble<D, E, F, P, R, X>(
	id: Uuid,
	message: Message,
	chat_class: String,
	portrait: P,
	bubble_color: String,
	disabled: D,
	editing: ReadSignal<Option<Uuid>>,
	set_editing: WriteSignal<Option<Uuid>>,
	mut edit: E,
	regenerate: Signal<Option<F>>,
	rewind: Signal<Option<R>>,
	delete: X
) -> impl IntoView
where
	D: Fn() -> bool + Clone + Send + Sync + 'static,
	E: FnMut(Uuid, String) + 'static,
	F: FnMut(Uuid) + Clone + Send + Sync + 'static,
	P: IntoView,
	R: FnMut(Uuid) + Clone + Send + Sync + 'static,
	X: FnMut(Uuid) + 'static
{
	let UseClipboardReturn {
		is_supported: can_copy,
		text: _,
		copied,
		copy
	} = use_clipboard();
	// A reference to the message editor, for focusing and selecting the text.
	let textarea = NodeRef::<html::Textarea>::new();
	// The content of the message editor.
	let (content, set_content) = signal(message.content);
	// Whether any editor is open.
	let editor_open = move || editing().is_some();
	// Whether the message is being edited.
	let editing = move || editing() == Some(id);
	view! {
		<div class={ format!("chat {}", chat_class) }>
			<div class="chat-image avatar">
				<div class="w-10">{ portrait.into_view() }</div>
			</div>
			<div class={
				format!(
					"
						chat-bubble
						{} xl-shadow
						whitespace-pre-wrap hyphens-auto
					",
					bubble_color
				)
			}>
				<MessageEditor
					id=id
					node_ref=textarea
					editing=editing
					message=content
					set_message=set_content
					close=move |id| {
						set_editing(None);
						edit(id, content());
					}
				/>
				{
					move || {
						if editing() { ().into_any() }
						else
						{
							view! {
								<div class="text-black">{content}</div>
							}.into_any()
						}
					}
				}
			</div>
			<div class="chat-footer">
				{
					let disabled = disabled.clone();
					view! {
						<Show when=move || regenerate().is_some()>
							<RegenerateButton
								id=id
								disabled={
									let disabled = disabled.clone();
									move || disabled() || editor_open()
								}
								click={regenerate().unwrap()}
							/>
						</Show>
					}
				}
				{
					let disabled = disabled.clone();
					view! {
						<Show when=move || rewind().is_some()>
							<RewindButton
								id=id
								disabled={
									let disabled = disabled.clone();
									move || disabled() || editor_open()
								}
								click={rewind().unwrap()}
							/>
						</Show>
					}
				}
				<EditButton
					id=id
					disabled={
						let disabled = disabled.clone();
						move || disabled() || (editor_open() && !editing())
					}
					click=move |id| {
						if !editor_open()
						{
							// Open the editor first, then focus it and place the
							// cursor at the end of the content.
							set_editing(Some(id));
							let textarea = textarea.get().unwrap();
							textarea.focus().unwrap();
							textarea.set_selection_start(
								Some(content().len() as u32)
							).unwrap();
						}
						else
						{
							set_editing(None);
						}
					}
				/>
				{
					// This is necessary because `disabled` does not pass
					// through the intermediate layers to the `CopyButton`.
					let disabled = disabled.clone();
					view! {
						<Show
							when=can_copy
						>
							<CopyButton
								id=id
								disabled={
									let disabled = disabled.clone();
									move || disabled() || editor_open()
								}
								click={
									let copy = copy.clone();
									move |_| copy(&content())
								}
								copied=copied
							/>
						</Show>
					}
				}
				<DeleteButton
					id=id
					disabled=move || disabled() || editor_open()
					click=delete
				/>
			</div>
		</div>
	}
}

/// Represents an incomplete assistant message.
///
/// # Arguments
///
/// * `message` - Obtains the message content.
#[component]
pub fn IncompleteAssistantMessage(message: ReadSignal<String>)
-> impl IntoView
{
	view! {
		<div class="chat chat-start ml-8">
			<div class="chat-image avatar">
				<div class="w-10"><AssistantImage/></div>
			</div>
			<div class="
				chat-bubble
				bg-green-300 xl-shadow
				whitespace-pre-wrap hyphens-auto
			">
				<div class="text-black">
					{message}
					<span class="loading loading-dots loading-xs"></span>
				</div>
			</div>
		</div>
	}
}

/// Represents a message editor.
///
/// # Arguments
///
/// * `id` - Specifies the message to edit.
/// * `node_ref` - Should retain the mounted `textarea`.
/// * `editing` - Specifies whether to display the component.
/// * `message` - Obtains the message content.
/// * `set_message` - Updates the message content.
/// * `close` - Is called when the editor is closed.
#[component]
pub fn MessageEditor<C, E>(
	id: Uuid,
	#[allow(unused_variables)] node_ref: NodeRef<html::Textarea>,
	editing: E,
	message: ReadSignal<String>,
	set_message: WriteSignal<String>,
	mut close: C
) -> impl IntoView
where
	C: FnMut(Uuid) + 'static,
	E: Fn() -> bool + Send + Sync + 'static
{
	let cols = 80;
	let rows = move || message().len() / cols + 1;
	view! {
		<textarea
			id={format!("message-editor-{id}")}
			node_ref=node_ref
			class="textarea textarea-secondary"
			rows=rows
			cols=cols
			hidden=move || !editing()
			on:input=move |ev| set_message(event_target_value(&ev))
			on:blur=move |_| close(id)
		>
			{message}
		</textarea>
	}
}

/// Represents a regenerate button used to regenerate an assistant message.
///
/// # Arguments
///
/// * `id` - Specifies the message to regenerate.
/// * `disabled` - Indicates whether the button should be disabled.
/// * `click` - A function that handles a click event, it accepts the `id`.
#[component]
pub fn RegenerateButton<D, F>(
	id: Uuid,
	disabled: D,
	mut click: F
) -> impl IntoView
where
	D: Fn() -> bool + Send + Sync + 'static,
	F: FnMut(Uuid) + 'static
{
	view! {
		<div
			class="tooltip tooltip-bottom"
			data-tip="Regenerate response"
		>
			<button
				class="btn btn-circle btn-ghost btn-xs disabled:opacity-25"
				disabled=disabled
				on:click=move |_| click(id)
			>
				<RegenerateImage />
			</button>
		</div>
	}
}

/// Represents a rewind button used to rewind the conversation to a specified
/// message.
///
/// # Arguments
///
/// * `id` - Specifies the message to rewind to.
/// * `disabled` - Indicates whether the button should be disabled.
/// * `click` - A function that handles a click event, it accepts the `id`.
#[component]
pub fn RewindButton<D, R>(id: Uuid, disabled: D, mut click: R) -> impl IntoView
where
	D: Fn() -> bool + Send + Sync + 'static,
	R: FnMut(Uuid) + 'static
{
	view! {
		<div
			class="tooltip tooltip-bottom"
			data-tip="Rewind conversation"
		>
			<button
				class="btn btn-circle btn-ghost btn-xs disabled:opacity-25"
				disabled=disabled
				on:click=move |_| click(id)
			>
				<RewindImage />
			</button>
		</div>
	}
}

/// Represents an edit button used to edit a message.
///
/// # Arguments
///
/// * `id` - Specifies the message to edit.
/// * `disabled` - Indicates whether the button should be disabled.
/// * `click` - A function that handles a click event, it accepts the `id`.
#[component]
pub fn EditButton<D, E>(id: Uuid, disabled: D, mut click: E) -> impl IntoView
where
	D: Fn() -> bool + Clone + Send + Sync + 'static,
	E: FnMut(Uuid) + 'static
{
	view! {
		<div
			class="tooltip tooltip-bottom"
			data-tip="Edit message"
		>
			<button
				class="btn btn-circle btn-ghost btn-xs disabled:opacity-25"
				disabled=disabled
				on:click=move |_| click(id)
			>
				<EditImage />
			</button>
		</div>
	}
}

/// Represents a copy button used to copy a message to the clipboard.
///
/// # Arguments
///
/// * `id` - Specifies the message to copy.
/// * `disabled` - Indicates whether the button should be disabled.
/// * `click` - A function that handles a click event, it accepts the `id`.
/// * `copied` - A signal that indicates whether the message has been copied.
#[component]
pub fn CopyButton<C, D>(
	id: Uuid,
	disabled: D,
	mut click: C,
	copied: Signal<bool>
) -> impl IntoView
where
	C: FnMut(Uuid) + 'static,
	D: Fn() -> bool + Clone + Send + Sync + 'static
{
	view! {
		<div
			class="tooltip tooltip-bottom"
			data-tip="Copy message"
		>
			<button
				class="btn btn-circle btn-ghost btn-xs disabled:opacity-25"
				disabled=disabled
				on:click=move |_| click(id)
			>
				<Show
					when=copied
					fallback=move || view! { <CopyImage /> }
				>
					<CopiedImage />
				</Show>
			</button>
		</div>
	}
}

/// Represents a delete button used to delete a message.
///
/// # Arguments
///
/// * `id` - Specifies the message to delete.
/// * `disabled` - Indicates whether the button should be disabled.
/// * `click` - A function that handles a click event, it accepts the `id`.
#[component]
pub fn DeleteButton<D, X>(id: Uuid, disabled: D, mut click: X) -> impl IntoView
where
	D: Fn() -> bool + Clone + Send + Sync + 'static,
	X: FnMut(Uuid) + 'static
{
	view! {
		<div
			class="tooltip tooltip-bottom"
			data-tip="Delete message"
		>
			<button
				class="btn btn-circle btn-ghost btn-xs disabled:opacity-25"
				disabled=disabled
				on:click=move |_| click(id)
			>
				<DeleteImage />
			</button>
		</div>
	}
}

////////////////////////////////////////////////////////////////////////////////
//                               System prompt.                               //
////////////////////////////////////////////////////////////////////////////////

/// Get the system prompt to use for the chat completion.
#[cfg(feature = "ssr")]
fn get_system_prompt() -> String
{
	let path =
		std::env::var("SYSTEM_PROMPT").expect("SYSTEM_PROMPT must be set");
	let bytes = std::fs::read(path).expect("Failed to read system prompt");
	String::from_utf8_lossy(&bytes).to_string()
}

/// Get a [system message](Message) with the [system prompt](get_system_prompt).
///
/// # Arguments
///
/// * `delay` - The delay to use to test the loading state of the chat. This
///   should ordinarily be `None` in production.
#[server(SystemMessageFn)]
pub async fn system_message(
	delay: Option<Duration>
) -> Result<Message, ServerFnError>
{
	if let Some(delay) = delay
	{
		// The delay is used to test the loading state of the chat.
		tokio::time::sleep(delay).await;
	}
	Ok(Role::System.message(get_system_prompt()))
}
