# Chatterverse OpenAI API
Welcome to the Chatterverse OpenAI API, a Rust library that provides a convenient way to interact with the OpenAI GPT-3 model for generating chat completions. This README will guide you through using this API to harness the power of the GPT-3 language model.

## 1. Introduction
The Chatterverse OpenAI API is a Rust library designed to simplify the integration of OpenAI's GPT-3 language model into your applications. It provides an easy-to-use interface for generating chat completions with the GPT-3 model, allowing you to have interactive and dynamic conversations with the AI.

## 2. Getting Started
Before you can use this API, you'll need to set up a Rust project and add this library as a dependency. Here's how you can get started:

## Installation
Add this library to your Cargo.toml file:

Copy code

```toml
[dependencies]
chatterverse-openai = {git = "https://github.com/chatterverse-ai/openai-client.git"}
```

## Usage
Import the library and create a Client instance with your OpenAI API key:


Copy code

```rust
use chatterverse_openai::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "YOUR_OPENAI_API_KEY";
    let client = Client::new(api_key)?;

    // Use the client to generate chat completions (see examples below).

    Ok(())
}
```

The primary functionality of this library revolves around generating chat completions using the GPT-3 model. Here's how you can use it:

### Creating a Client
Copy code:

```rust
use chatterverse_openai::Client;

let api_key = "YOUR_OPENAI_API_KEY";
let client = Client::new(api_key)?;
```

### Generating Chat Completions
To generate chat completions, you can use the chat_completions method of the Client:

```rust
let model = "gpt-3.5-turbo"; // The model you want to use
let conversation = vec![
    "User: What is the capital of France?",
    "AI: The capital of France is Paris."
];

let response = client.chat_completions(model)
    .messages(conversation)
    .generate()
    .await?;

println!("AI's response: {}", response.choices[0].text);
Please refer to the chatterverse-openai documentation for more details and advanced usage.
```

### Error Handling
This library uses the [`anyhow`](https://github.com/chatterverse-ai/openai-client.git) crate for error handling. Errors are returned as OpenAiError types, which include detailed error messages to help you diagnose and fix issues that may arise during API interactions.

### Contributing
We welcome contributions to improve this library and make it more useful for the community. If you'd like to contribute, please follow our contributing guidelines.

## License
This library is licensed under the MIT License. You are free to use, modify, and distribute it as per the terms of the license.

For more information, please refer to the [LICENSE](./LICENSE) file

Thank you for using the Chatterverse OpenAI API! If you have any questions or need assistance, feel free to open an issue on this repo 