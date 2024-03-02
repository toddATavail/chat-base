#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main()
{
	use axum::Router;
	use axum::routing::get;
	use chat_base::app::{App, shell};
	use chat_base::chat::chat_handler;
	use dotenvy::dotenv;
	use leptos::prelude::*;
	use leptos_axum::{LeptosRoutes, generate_route_list};
	use tracing::{Level, debug, info};
	use tracing_subscriber::{EnvFilter, FmtSubscriber};

	let filter_level = Level::ERROR;
	let level = Level::TRACE;
	let env_filter = EnvFilter::builder()
		.with_default_directive(Level::ERROR.into())
		.from_env()
		.expect("Failed to read from the environment")
		.add_directive(format!("chat_base={level}").parse().unwrap());
	let subscriber = FmtSubscriber::builder()
		.with_env_filter(env_filter)
		.with_file(true)
		.with_line_number(true)
		.finish();
	tracing::subscriber::set_global_default(subscriber)
		.expect("setting default subscriber failed");
	debug!(
		"Logging online: chat_base=≥{} [others={}]",
		level, filter_level
	);

	dotenv().expect("Failed to load .env file");
	debug!(
		"Environment:{}",
		std::env::vars()
			.map(|(k, v)| format!("\n\t{}={}", k, v))
			.collect::<Vec<_>>()
			.join("")
	);

	// Setting get_configuration(None) means we'll be using cargo-leptos's env
	// values For deployment these variables are:
	// <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
	// Alternately a file can be specified such as Some("Cargo.toml") The file
	// would need to be included with the executable when moved to deployment
	let conf = get_configuration(None).unwrap();
	let leptos_options = conf.leptos_options;
	debug!("{:#?}", leptos_options);

	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);

	// Build the application from its routes and the configured Leptos options.
	let app = Router::new()
		.route("/api/chat", get(chat_handler))
		.leptos_routes(&leptos_options, routes, {
			let leptos_options = leptos_options.clone();
			move || shell(leptos_options.clone())
		})
		.fallback(leptos_axum::file_and_error_handler(shell))
		.with_state(leptos_options);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	info!("Listening on http://{}", &addr);
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main()
{
	// No need for this on the client, since a purely client-side experience is
	// not possible with this app.
}
