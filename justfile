# Run `leptos` in watch mode. Handles Tailwind automatically. Note that
# RUSTFLAGS must be set here, not in `Cargo.toml`, because `leptos` apparently
# does not respect `Cargo.toml`'s `RUSTFLAGS`.
watch:
	RUSTFLAGS=--cfg=web_sys_unstable_apis LEPTOS_TAILWIND_VERSION=v4.1.1 cargo leptos watch --hot-reload

# Open the browser to the local server started by `cargo leptos`.
open:
	open http://localhost:3000

# Lint the project. Valid options of `feature` right now are:
#   - `hydrate`: client
#   - `ssr`: server
lint feature:
	cargo clean
	RUSTFLAGS=--cfg=web_sys_unstable_apis cargo clippy --features={{feature}} --no-deps

# Lint both the server and the client.
lint-all:
	cargo clean
	RUSTFLAGS=--cfg=web_sys_unstable_apis cargo clippy --features=ssr --no-deps
	cargo clean
	RUSTFLAGS=--cfg=web_sys_unstable_apis cargo clippy --features=hydrate --no-deps

# Clean the project. It's good to do this when not engaged in active
# development, because the binaries can get quite large.
clean:
	cargo clean
