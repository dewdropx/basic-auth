# HTTP Basic Authentication in Rust

A simple Rust project demonstrating how to send an HTTP GET request with **Basic Authentication** using the `reqwest` blocking HTTP client. The project uses `httpbin.org` to test the request and prints the resulting HTTP response.

## Features

- **Blocking HTTP requests** using `reqwest::blocking::Client`.
- **Basic Authentication** using the `.basic_auth()` method.
- **Optional password handling** using Rust's `Option<String>`.
- **Error propagation** using `Result` and the `?` operator.
- **HTTP response inspection** using Rust's debug formatting.

## Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/) — HTTP client library.
- [HTTPBin](https://httpbin.org/) — HTTP testing service.

## How It Works

1. **Initialize the HTTP client:** Create a blocking `reqwest::blocking::Client`.
2. **Define credentials:** Specify the username and an optional password.
3. **Configure Basic Authentication:** Use `.basic_auth(user, passwd)` to attach the authentication credentials to the request.
4. **Send the GET request:** Call `.send()?` to execute the request and handle any request errors.
5. **Print the response:** Display the response using Rust's `Debug` formatting.

## Prerequisites

Install Rust and Cargo using [rustup](https://rustup.rs/).

Verify the installation:

```bash
rustc --version
cargo --version
```

## Installation and Usage

### 1. Clone the repository

```bash
git clone https://github.com/dewdropx/basic-auth.git
cd basic-auth
```

### 2. Configure dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.13", features = ["blocking"] }
```

### 3. Run the project

```bash
cargo run
```

## Example Output

The program prints a debug representation of the HTTP response, including response metadata such as the status code and headers. The exact output may vary.

A successful request typically returns:

```text
Response {
    status: 200 OK,
    ...
}
```

The response body is not explicitly printed by this program.
