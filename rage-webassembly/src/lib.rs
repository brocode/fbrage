mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use js_sys::Array;

use std::str::FromStr;

mod ageutil;
mod error;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn encrypt_message(message: String, public_keys: Array) -> Result<String, String> {
    set_panic_hook();
    let public_key_strings: Vec<String> = public_keys
        .iter()
        .map(|val| val.as_string().ok_or("Not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    let recipients = ageutil::parse_public_keys(public_key_strings)?;
    let encrypted_message = ageutil::encrypt_message(recipients, &message)?;
    Ok(encrypted_message)
}

#[wasm_bindgen]
pub fn decrypt_message(message: String, private_keys: Array) -> Result<String, String> {
    set_panic_hook();
    let private_key_strings: Vec<String> = private_keys
        .iter()
        .map(|val| val.as_string().ok_or("Not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    let identities = ageutil::parse_private_keys(private_key_strings)?;
    let decrypted_message = ageutil::decrypt_message(identities, &message)?;
    Ok(decrypted_message)
}

#[wasm_bindgen]
pub fn greet() -> Result<String, String> {
    set_panic_hook();
    let key = "age1hwuyjjjvljra6j00vynkgaxap7zlh4fmadj09m4jn6t9t0nveyds6mt6zf";
    let recipient = Box::new(age::x25519::Recipient::from_str(key)?);

    let plaintext = "Hello world!";

    let encrypted = ageutil::encrypt_message(vec![recipient], plaintext)?;
    log("Encryption success");
    log(&encrypted);
    Ok(encrypted)
}
