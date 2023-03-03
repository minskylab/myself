# Myself

Myself is a Rust library for building conversational agents powered by OpenAI's language model. It provides a simple Agent abstraction to manage interactions and memory, making it easy to create natural language interfaces for various applications.

## Example

You need to set the `OPENAI_API_KEY` environment variable to your OpenAI API key. You can get one [here](https://beta.openai.com/account/api-keys).

```rust
use dotenvy::dotenv;
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut agent = AgentBuilder::new()
        .with_name("AI".to_string())
        .build()
        .await;

    let message = "Hello World".to_string();
    let response = agent.interact_with_default(&message).await.unwrap();

    println!("{}", response);
}
```

## A New Kind of Firmware

Myself can be thought of as a new kind of firmware for large language models. In traditional computing systems, firmware is software that is embedded in hardware devices to control their behavior. Similarly, Myself provides a layer of software abstraction that sits between the language model and the application, managing interactions and memory in a way that is efficient and reliable.

## Features

- Simple API for creating and managing conversational agents
- Efficient memory management for large language models
- Integration with OpenAI's API for easy access to cutting-edge language technology
