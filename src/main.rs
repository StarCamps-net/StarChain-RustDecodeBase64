extern crate base64;
extern crate reqwest;

use base64::decode;
use reqwest::blocking::get;
use std::io::{self, Write};
use std::str;

fn decode_base64(encoded: &str) -> Result<String, base64::DecodeError> {
    let decoded_bytes = decode(encoded)?;
    let decoded_str = str::from_utf8(&decoded_bytes).map_err(|_| base64::DecodeError::InvalidByte(0, 0))?;
    Ok(decoded_str.to_string())
}

fn main() {
    // Enter a StarChain Base64 encoded value
    print!("Enter a StarChain Base64 encoded value: ");
    io::stdout().flush().unwrap(); 

    // Read the input
    let mut encoded_input = String::new();
    io::stdin().read_line(&mut encoded_input).expect("Failed to read line");
    let encoded_input = encoded_input.trim(); 

    // Decode the input
    match decode_base64(encoded_input) {
        Ok(decoded) => {
            println!("Decoded: {}", decoded);

            // Make an HTTP GET request to the decoded URL
            match get(&decoded) {
                Ok(response) => {
                    if let Ok(body) = response.text() {
                        println!("HTTP Response: {}", body);
                    } else {
                        println!("Failed to read the response body");
                    }
                }
                Err(e) => println!("Failed to make HTTP request: {}", e),
            }
        }
        Err(e) => println!("Failed to decode: {}", e),
    }
}