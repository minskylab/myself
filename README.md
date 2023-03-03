# Myself

Myself is a Rust library for building conversational agents powered by OpenAI's language model. It provides a simple Agent abstraction to manage interactions and memory, making it easy to create natural language interfaces for various applications.

## ⚠️ Warning

Please note that `Myself` is currently under development and is not yet suitable for production use. While you are welcome to try it out and provide feedback, we caution that it may have an incomplete implementation and may not function as intended. Our top priorities at this time are to improve the documentation and complete the list of features.

## Features

- [x] Simple Agent abstraction.
- [x] Manage interactions and memory.
- [x] Support SQLite database.
- [x] Support GPT-3 OpenAI's language model.
- [ ] Support other SQL databases (e.g. PostgreSQL and MySQL).
- [ ] Support other language models (we plan to add support for other open source large language models).
- [ ] Improve documentation, add more examples.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
myself = "0.1.0"
```

Or use cargo:

```bash
cargo add myself
```

## Example Usage

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

You can run the other examples in the `examples` directory. Only clone the repository and run one of [minimal](/examples/minimal/main.rs), [complex](/examples/complex/main.rs), [graphql](/examples/graphql/main.rs) or [neural-linux](/examples/neural-linux/main.rs) examples:

```bash
cargo run --example <example_name>
```

## How it works

The main idea behind Myself is to provide a simple abstraction for building conversational agents. The `Agent` abstraction manages interactions and memory, making it easy to create natural language interfaces for various applications.

An `Agent` is a individual that can interact with the world. It has a name, a personality (a.k.a. constitution), and a memory. The name is used to address the agent, the personality is used to generate responses, and the memory is used to keep track of the context of the conversation.

In `myself` the `Agent` abstraction is implemented by the `Agent` struct. The `Agent` struct is responsible for managing interactions and memory. It is also responsible for generating responses using the OpenAI language model. The `Agent` struct is initialized with a `AgentBuilder` struct, which is used to configure the agent.

For example, if you want to create an agent named Bob, you can do the following:

```rust
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    let mut agent = AgentBuilder::new()
        .with_name("Bob".to_string())
        .build()
        .await;
}
```

For example, you can set the constitution and the default user name of the agent:

```rust
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    let mut agent = AgentBuilder::new()
        .with_name("Linux Server".to_string())
        .with_default_user_name("User".to_string())
        .with_default_constitution("I want you to act as a linux terminal. I will type commands and you will reply with what the terminal should show. I want you to only reply with the terminal output inside one unique code block, and nothing else. do not write explanations. do not type commands unless I instruct you to do so. When I need to tell you something in English, I will do so by putting text inside curly brackets {like this}.".into())
        .with_default_memory_size(50)
        .build()
        .await;
}
```

In the example above, we set the name of the agent to `Linux Server`, the default user name is `User`, the constitution to a string that describes the personality of the agent, and the default memory size to 50.

The `Agent` provides a methods to interact in different instances. For example, you can init (create) a new interaction with the agent:

```rust
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    let mut agent = AgentBuilder::new()
            .with_name("AI (Agent)".to_string())
            .build()
            .await;

    let interaction = agent
        .init_interaction(
            "Joe (Human)".to_string(),
            "A talkative chatbot conversation".to_string(),
            40,
        )
        .await;
}
```

And you can interact with it using the `interact` method:

```rust
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    let mut agent = AgentBuilder::new()
            .with_name("AI (Agent)".to_string())
            .build()
            .await;

    let interaction = agent
        .init_interaction(
            "Joe (Human)".to_string(),
            "A talkative chatbot conversation".to_string(),
            40,
        )
        .await;

    let message = "How are you?, explain please".to_string();
    let response = agent.interact_with(interaction.id, &message).await.unwrap();

    println!("{}", response);
}
```

In the example above, we create a new interaction with the agent, and then we interact with it using the `interact` method. The `interact` method takes the interaction id and the message as arguments, and returns a response.

The `Agent` also provides a method to interact with the default interaction:

```rust
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    let mut agent = AgentBuilder::new()
            .with_name("AI (Agent)".to_string())
            .build()
            .await;

    let message = "How are you?, explain please".to_string();
    let response = agent.interact_with_default(&message).await.unwrap();

    println!("{}", response);
}
```

The `Interaction` structure have the following form:

```rust
struct Interaction {
    pub id: Uuid,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,

    pub user_name: String,

    pub long_term_memory: String,
    pub short_term_memory: Option<String>,

    pub short_term_memory_size: usize,
}
```

The `user_name` field is used to address the user in the conversation. The `long_term_memory` field is used to store the constitution of the the agent for this interaction. The `short_term_memory` field is used to store the last messages of the conversation in a buffer represented by a string. The `short_term_memory_size` field is used to set the size of the buffer measured as the number of lines (separated by '\n') in the `dynamic_memory`.
