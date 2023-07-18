mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use age::{self, Recipient};
use age::armor::{ArmoredWriter, Format};

use std::io::{Read, Write};
use std::iter;
use std::str::FromStr;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    set_panic_hook();
    let key = "age1hwuyjjjvljra6j00vynkgaxap7zlh4fmadj09m4jn6t9t0nveyds6mt6zf";
    let recipient = Box::new(age::x25519::Recipient::from_str(key).unwrap());

    let plaintext = b"Hello world!";

    // Encrypt the plaintext to a ciphertext...
    let encrypted = {
        let mut output_vec = Vec::new();
        let mut output = ArmoredWriter::wrap_output(&mut output_vec, Format::AsciiArmor).unwrap();

        let encryptor = age::Encryptor::with_recipients(vec![recipient])
            .expect("we provided a recipient");

        let mut writer = encryptor.wrap_output(&mut output).unwrap();
        writer.write_all(plaintext).unwrap();
        writer.finish().unwrap();
        output.finish().unwrap();



        String::from_utf8(output_vec).unwrap()
    };
    log("Encryption success");
    log(&encrypted);
}
