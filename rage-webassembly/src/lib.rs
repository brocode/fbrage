mod utils;

use age;
use age::armor::{ArmoredWriter, Format};
use ageutil::encrypt_message;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use js_sys::Array;
use std::io::Write;
use std::str::FromStr;

mod ageutil;
mod error;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn decrypt(private_keys: Array) -> Result<String, String> {
    let private_key_strings: Vec<String> = private_keys
        .iter()
        .map(|val| val.as_string().ok_or("Not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    ageutil::parse_private_keys(private_key_strings)?;
    return Ok("".to_string());
}

#[wasm_bindgen]
pub fn greet() -> Result<String, String> {
    set_panic_hook();
    let key = "age1hwuyjjjvljra6j00vynkgaxap7zlh4fmadj09m4jn6t9t0nveyds6mt6zf";
    let recipient = Box::new(age::x25519::Recipient::from_str(key)?);

    let plaintext = "Hello world!";

    let encrypted = encrypt_message(plaintext, vec![recipient])?;
    log("Encryption success");
    log(&encrypted);
    return Ok(encrypted);
}
