use leptos::prelude::*;

////////////////////////////////////////////////////////////////////////////////
//                             Image components.                              //
////////////////////////////////////////////////////////////////////////////////

/// The assistant's image. This is used to represent the assistant in the chat.
/// This is the solid "cpu-chip" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn AssistantImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-green-500"
			role="img"
		>
			<title>"Assistant says…"</title>
			  <path d="M16.5 7.5h-9v9h9v-9Z" />
			  <path
				fill-rule="evenodd"
				d="M8.25 2.25A.75.75 0 0 1 9 3v.75h2.25V3a.75.75 0 0 1 1.5
					0v.75H15V3a.75.75 0 0 1 1.5 0v.75h.75a3 3 0 0 1 3
					3v.75H21A.75.75 0 0 1 21 9h-.75v2.25H21a.75.75 0 0 1 0
					1.5h-.75V15H21a.75.75 0 0 1 0 1.5h-.75v.75a3 3 0 0 1-3
					3h-.75V21a.75.75 0 0 1-1.5 0v-.75h-2.25V21a.75.75 0 0 1-1.5
					0v-.75H9V21a.75.75 0 0 1-1.5 0v-.75h-.75a3 3 0 0
					1-3-3v-.75H3A.75.75 0 0 1 3 15h.75v-2.25H3a.75.75 0 0 1
					0-1.5h.75V9H3a.75.75 0 0 1 0-1.5h.75v-.75a3 3 0 0 1
					3-3h.75V3a.75.75 0 0 1 .75-.75ZM6 6.75A.75.75 0 0 1 6.75
					6h10.5a.75.75 0 0 1 .75.75v10.5a.75.75 0 0
					1-.75.75H6.75a.75.75 0 0 1-.75-.75V6.75Z"
				clip-rule="evenodd"
			/>
		</svg>
	}
}

/// The user's image. This is used to represent the user in the chat. This is
/// the solid "user" from the [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn UserImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-sky-500"
			role="img"
		>
			<title>"User says…"</title>
			<path
				fill-rule="evenodd"
				d="M7.5 6a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0ZM3.751
					20.105a8.25 8.25 0 0 1 16.498 0 .75.75 0 0
					1-.437.695A18.683 18.683 0 0 1 12 22.5c-2.786
					0-5.433-.608-7.812-1.7a.75.75 0 0 1-.437-.695Z"
				clip-rule="evenodd"
			/>
		</svg>
	}
}

/// The system's image. This is used to represent the system message in the
/// chat. This is the solid "scales" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn SystemImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-slate-700"
			role="img"
		>
			<title>"System prompt is…"</title>
			<path
				fill-rule="evenodd"
				d="M12 2.25a.75.75 0 0 1 .75.75v.756a49.106 49.106 0 0 1 9.152 1
					.75.75 0 0 1-.152 1.485h-1.918l2.474 10.124a.75.75 0 0
					1-.375.84A6.723 6.723 0 0 1 18.75 18a6.723 6.723 0 0
					1-3.181-.795.75.75 0 0
					1-.375-.84l2.474-10.124H12.75v13.28c1.293.076 2.534.343
					3.697.776a.75.75 0 0 1-.262 1.453h-8.37a.75.75 0 0
					1-.262-1.453c1.162-.433 2.404-.7 3.697-.775V6.24H6.332l2.474
					10.124a.75.75 0 0 1-.375.84A6.723 6.723 0 0 1 5.25 18a6.723
					6.723 0 0 1-3.181-.795.75.75 0 0 1-.375-.84L4.168
					6.241H2.25a.75.75 0 0 1-.152-1.485 49.105 49.105 0 0 1
					9.152-1V3a.75.75 0 0 1 .75-.75Zm4.878 13.543 1.872-7.662 1.872
					7.662h-3.744Zm-9.756 0L5.25 8.131l-1.872 7.662h3.744Z"
				clip-rule="evenodd"
			/>
		</svg>
	}
}

/// The regenerate image. This indicates that an assistant message can be
/// regenerated. This is the solid "arrow-path" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn RegenerateImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-secondary"
			role="img"
		>
			<title>Regenerate response</title>
			<path
				fillRule="evenodd"
				d="M4.755 10.059a7.5 7.5 0 0 1 12.548-3.364l1.903
					1.903h-3.183a.75.75 0 1 0 0 1.5h4.992a.75.75 0 0 0
					.75-.75V4.356a.75.75 0 0 0-1.5 0v3.18l-1.9-1.9A9 9 0 0 0
					3.306 9.67a.75.75 0 1 0 1.45.388Zm15.408 3.352a.75.75 0 0
					0-.919.53 7.5 7.5 0 0 1-12.548
					3.364l-1.902-1.903h3.183a.75.75 0 0 0 0-1.5H2.984a.75.75 0
					0 0-.75.75v4.992a.75.75 0 0 0 1.5 0v-3.18l1.9 1.9a9 9 0 0 0
					15.059-4.035.75.75 0 0 0-.53-.918Z"
				clipRule="evenodd"
			/>
		</svg>
	}
}

/// The rewind image. This indicates that the conversation can be rewound to the
/// marked position. This is the solid "backward" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn RewindImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-secondary"
			role="img"
		>
			<title>Rewind conversation</title>
			<path
				d="M9.195 18.44c1.25.714 2.805-.189 2.805-1.629v-2.34l6.945
					3.968c1.25.715 2.805-.188
					2.805-1.628V8.69c0-1.44-1.555-2.343-2.805-1.628L12
					11.029v-2.34c0-1.44-1.555-2.343-2.805-1.628l-7.108
					4.061c-1.26.72-1.26 2.536 0 3.256l7.108 4.061Z"
			/>
		</svg>
	}
}

/// The edit image. This indicates that the message can be edited _in situ_.
/// This is the solid "pencil" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn EditImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-secondary"
		>
			<title>Edit message</title>
			<path
				d="M21.731 2.269a2.625 2.625 0 0 0-3.712 0l-1.157 1.157 3.712
					3.712 1.157-1.157a2.625 2.625 0 0 0 0-3.712ZM19.513
					8.199l-3.712-3.712-12.15 12.15a5.25 5.25 0 0 0-1.32
					2.214l-.8 2.685a.75.75 0 0 0 .933.933l2.685-.8a5.25 5.25 0
					0 0 2.214-1.32L19.513 8.2Z"
			/>
		</svg>
	}
}

/// The copy image. This indicates that the message can be copied to the
/// clipboard. This is the solid "clipboard-document" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn CopyImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-accent"
		>
			<path
				fill-rule="evenodd"
				d="M17.663 3.118c.225.015.45.032.673.05C19.876 3.298 21 4.604
					21 6.109v9.642a3 3 0 0 1-3
					3V16.5c0-5.922-4.576-10.775-10.384-11.217.324-1.132
					1.3-2.01 2.548-2.114.224-.019.448-.036.673-.051A3 3 0 0 1
					13.5 1.5H15a3 3 0 0 1 2.663 1.618ZM12 4.5A1.5 1.5 0 0 1
					13.5 3H15a1.5 1.5 0 0 1 1.5 1.5H12Z"
				clip-rule="evenodd"
			/>
			<path
				d="M3 8.625c0-1.036.84-1.875 1.875-1.875h.375A3.75 3.75 0 0 1 9
					10.5v1.875c0 1.036.84 1.875 1.875 1.875h1.875A3.75 3.75 0 0
					1 16.5 18v2.625c0 1.035-.84 1.875-1.875 1.875h-9.75A1.875
					1.875 0 0 1 3 20.625v-12Z"
			/>
			<path
				d="M10.5 10.5a5.23 5.23 0 0 0-1.279-3.434 9.768 9.768 0 0 1
				6.963 6.963 5.23 5.23 0 0 0-3.434-1.279h-1.875a.375.375 0 0
				1-.375-.375V10.5Z"
			/>
	  </svg>
	}
}

/// The copied image. This indicates that the message has been copied to the
/// clipboard. This is the solid "clipboard-document-check" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn CopiedImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-success"
		>
			<path
				fill-rule="evenodd"
				d="M7.502 6h7.128A3.375 3.375 0 0 1 18 9.375v9.375a3 3 0 0 0
					3-3V6.108c0-1.505-1.125-2.811-2.664-2.94a48.972 48.972 0 0
					0-.673-.05A3 3 0 0 0 15 1.5h-1.5a3 3 0 0 0-2.663
					1.618c-.225.015-.45.032-.673.05C8.662 3.295 7.554 4.542
					7.502 6ZM13.5 3A1.5 1.5 0 0 0 12 4.5h4.5A1.5 1.5 0 0 0 15
					3h-1.5Z"
				clip-rule="evenodd"
			/>
			<path
				fill-rule="evenodd"
				d="M3 9.375C3 8.339 3.84 7.5 4.875 7.5h9.75c1.036 0 1.875.84
					1.875 1.875v11.25c0 1.035-.84 1.875-1.875 1.875h-9.75A1.875
					1.875 0 0 1 3 20.625V9.375Zm9.586 4.594a.75.75 0 0
					0-1.172-.938l-2.476 3.096-.908-.907a.75.75 0 0 0-1.06
					1.06l1.5 1.5a.75.75 0 0 0 1.116-.062l3-3.75Z"
				clip-rule="evenodd"
			/>
		</svg>
	}
}

/// The delete image. This indicates that the message can be deleted. This is
/// the solid "trash" from the
/// [`heroicons`](https://heroicons.com/solid) set.
#[component]
pub fn DeleteImage() -> impl IntoView
{
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			class="w-6 h-6 text-warning"
		>
			<title>Delete message</title>
			<path
				fill-rule="evenodd"
				d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1
					1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991
					2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75
					0 0 1-.256-1.478A48.567 48.567 0 0 1 7.5
					4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0
					1 3.369 0c1.603.051 2.815 1.387 2.815
					2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05
					15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6
					0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0
					1 0-1.5.058l.347 9a.75.75 0 1 0
					1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347
					9a.75.75 0 0 0 1.5.058l.345-9Z"
				clip-rule="evenodd"
			/>
		</svg>
	}
}
