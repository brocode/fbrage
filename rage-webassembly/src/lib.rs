mod utils;

use age;
use age::armor::{ArmoredWriter, Format};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use js_sys::Array;
use std::io::Write;
use std::str::FromStr;

mod ageutil;

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

    let plaintext = b"Hello world!";

    // Encrypt the plaintext to a ciphertext...
    let encrypted = {
        let mut output_vec = Vec::new();
        let mut output = ArmoredWriter::wrap_output(&mut output_vec, Format::AsciiArmor)
            .map_err(|e| e.to_string())?;

        let encryptor =
            age::Encryptor::with_recipients(vec![recipient]).ok_or("Cannot create recipients")?;

        let mut writer = encryptor
            .wrap_output(&mut output)
            .map_err(|e| e.to_string())?;
        writer.write_all(plaintext).unwrap();
        writer.finish().map_err(|e| e.to_string())?;
        output.finish().map_err(|e| e.to_string())?;

        String::from_utf8(output_vec).map_err(|e| e.to_string())?
    };
    log("Encryption success");
    log(&encrypted);
    return Ok(encrypted);
}
