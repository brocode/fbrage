mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use js_sys::Array;

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
    let recipients = ageutil::parse_public_keys(&public_key_strings)?;
    let encrypted_message = ageutil::encrypt_message(recipients, &message)?;
    Ok(encrypted_message)
}
#[wasm_bindgen]
pub fn encrypt_file(file: &[u8], public_keys: Array) -> Result<Vec<u8>, String> {
    set_panic_hook();
    let public_key_strings: Vec<String> = public_keys
        .iter()
        .map(|val| val.as_string().ok_or("Not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    let recipients = ageutil::parse_public_keys(&public_key_strings)?;
    let encrypted_message = ageutil::encrypt_file(recipients, file)?;
    Ok(encrypted_message)
}

#[wasm_bindgen]
pub fn decrypt_message(message: String, private_keys: Array) -> Result<String, String> {
    set_panic_hook();
    let private_key_strings: Vec<String> = private_keys
        .iter()
        .map(|val| val.as_string().ok_or("Not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    let identities = ageutil::parse_private_keys(&private_key_strings)?;
    let decrypted_message = ageutil::decrypt_message(identities, &message)?;
    Ok(decrypted_message)
}

#[wasm_bindgen]
pub fn decrypt_file(file: &[u8], private_keys: Array) -> Result<Vec<u8>, String> {
    set_panic_hook();
    let private_key_strings: Vec<String> = private_keys
        .iter()
        .map(|val| val.as_string().ok_or("Not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    let identities = ageutil::parse_private_keys(&private_key_strings)?;
    let decrypted_message = ageutil::decrypt_file(identities, file)?;
    Ok(decrypted_message)
}

#[wasm_bindgen]
pub fn genkey() -> Result<JsValue, String> {
    set_panic_hook();
    let js_value = serde_wasm_bindgen::to_value(&ageutil::gen_key()).map_err(|e| e.to_string())?;
    Ok(js_value)
}
#[wasm_bindgen]
pub fn get_public_key_for_private_key(private_key: String) -> Result<String, String> {
    Ok(ageutil::get_public_key_for_private_key(&private_key)?)
}
