use crate::{
	chat::Chat,
	error_template::{AppError, ErrorTemplate}
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
	StaticSegment,
	components::{Route, Router, Routes}
};

/// The application shell.
pub fn shell(options: LeptosOptions) -> impl IntoView
{
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8"/>
				<meta
					name="viewport"
					content="width=device-width, initial-scale=1"
				/>
				<AutoReload options=options.clone()/>
				<HydrationScripts options/>
				<link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
				<MetaTags/>
			</head>
			<body>
				<App/>
			</body>
		</html>
	}
}

/// The main application component.
#[component]
pub fn App() -> impl IntoView
{
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		// Set the daisyUI theme.
		<Html attr:data-theme="dark"/>

		// sets the document title
		<Title text="Chat Base"/>

		// Injects the main stylesheet into the <head>. The `id` attribute of
		// `leptos` is used to hot-reload the stylesheet in development mode.
		<Stylesheet id="leptos" href="/pkg/chat-base.css"/>

		<Router>
			<main>
				<Routes fallback=|| {
					let mut outside_errors = Errors::default();
					outside_errors.insert_with_default_key(AppError::NotFound);
					view! {
						<ErrorTemplate outside_errors/>
					}
					.into_view()
				}>
					<Route path=StaticSegment("") view=Chat/>
				</Routes>
			</main>
		</Router>
	}
}
