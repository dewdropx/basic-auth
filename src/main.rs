use reqwest::blocking::Client; // blocking http client
use reqwest::Error; // reqwest error type

fn main() -> Result<(), Error> { // may return error instead of panicking

    // initialize an http client to send request
    let client = Client::new();

    // define user and passwd (if any)
    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    // send a get request and store response
    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send()?;

    // print the response if request succeeds
    println!("{:?}", response);
    
    Ok(())
}