use std::ops::Add;

pub fn base64_pad(encoded: String) -> String {
    String::from(encoded.to_owned()).add("=")
}