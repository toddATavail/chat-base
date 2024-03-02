# Chat Base

`Chat Base` is a simple web-based chat application, written in
[Rust](https://www.rust-lang.org/), using [Leptos](https://leptos.dev/) and
[Tailwind](https://tailwindcss.com/). It can talk to any local large language
model (LLM) server that provides an
[OpenAI-compatible&#32;API](https://platform.openai.com/docs/api-reference/introduction);
I primarily use it with [LM&#32;Studio](https://lmstudio.ai/). I developed it
late in 2023 when I was first using local models, but recently I decided to
modernize it and publish it on
[GitHub](https://github.com/toddATavail/chat-base).

# Environment

In order to run `Chat Base`, you will need to define several environment
variables. I recommend place an `.env` file in the checkout directory, like this
one:

```
OPENAI_API_URL=http://localhost:1234/v1
OPENAI_TOKEN=not-needed
SYSTEM_PROMPT=data/gm.system
```

* `OPENAI_API_URL`: Specifies the URL of the LLM server's API endpoint. The
  demo URL provided above corresponds to the default for LM Studio.
* `OPENAI_TOKEN`: Not needed for most local models, but can be used if your
  local model server has strange requirements.
* `SYSTEM_PROMPT`: Specifies the system prompt to be used by the LLM. Sample
  system prompts are provided in the `data` directory. Not every LLM uses a
  system prompt, so you can point this to an empty file if necessary.

# Running

To run `Chat Base`, you will need to have
[Rust](https://www.rust-lang.org/tools/install),
[Leptos](https://github.com/leptos-rs/cargo-leptos#getting-started), and
[Just](https://just.systems/) installed. Once you have done that, you can run
the following command:

```
just watch
```

This will occupy the foreground of the running terminal. You can then point your
browser to [`http://localhost:3000`](http://localhost:3000) to interact with the
application. (Don't click the link if you're looking at this on GitHub, as it's
a non-routable local address.)
