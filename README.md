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
        .with_name("Bob".to_string())
        .build()
        .await;

    let message = "Hello World".to_string();
    let response = agent.interact_with_default(&message).await.unwrap();

    println!("{}", response);
    // Hello there! How can I help you?
}
```

You can run the other examples in the `examples` directory. Only clone the repository and run one of [minimal](/examples/minimal/main.rs), [graphql](/examples/graphql/main.rs) or [neural-linux](/examples/neural-linux/main.rs) examples:

```bash
cargo run --example <example_name>
```

## A New Kind of Firmware

Myself can be thought of as a new kind of firmware for large language models. In traditional computing systems, firmware is software that is embedded in hardware devices to control their behavior. Similarly, Myself provides a layer of software abstraction that sits between the language model and the application, managing interactions and memory in a way that is efficient and reliable.
