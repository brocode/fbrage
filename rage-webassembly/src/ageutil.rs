use std::str::FromStr;
use std::sync::Arc;

use crate::error::AppError;
use serde::{Serialize, Deserialize};
use age::armor::ArmoredReader;
use age::armor::ArmoredWriter;
use age::armor::Format;
use age::Decryptor;
use age::Identity;
use age::Recipient;
use std::io::Write;
use std::io::{Cursor, Read};
use age::secrecy::ExposeSecret;

#[derive(Serialize, Deserialize)]
pub struct GeneratedKey {
    public_key: String,
    private_key: String,
}

pub fn encrypt_message(
    recipients: Vec<Box<dyn Recipient + Send>>,
    message: &str,
) -> Result<String, AppError> {
    let mut output_vec = Vec::new();
    let mut output = ArmoredWriter::wrap_output(&mut output_vec, Format::AsciiArmor)?;

    let encryptor =
        age::Encryptor::with_recipients(recipients).ok_or("Cannot create recipients")?;

    let mut writer = encryptor.wrap_output(&mut output)?;

    writer.write_all(message.as_bytes()).unwrap();
    writer.finish()?;
    output.finish()?;
    Ok(String::from_utf8(output_vec)?)
}

pub fn encrypt_file(
    recipients: Vec<Box<dyn Recipient + Send>>,
    file: &[u8],
) -> Result<Vec<u8>, AppError> {
    let mut output = Vec::new();

    let encryptor = age::Encryptor::with_recipients(recipients).ok_or("Cannot create recipients")?;

    let mut writer = encryptor.wrap_output(&mut output)?;

    writer.write_all(file).unwrap();
    writer.finish()?;
    Ok(output)
}

pub fn decrypt_message(
    identities: Vec<Arc<dyn Identity>>,
    message: &str,
) -> Result<String, AppError> {
    let input = Cursor::new(message);

    match Decryptor::new(ArmoredReader::new(input))? {
        Decryptor::Recipients(d) => {
            let mut reader = d.decrypt(identities.iter().map(|arc| arc.as_ref()))?;
            let mut buffer = String::new();
            reader.read_to_string(&mut buffer)?;
            Ok(buffer)
        }
        Decryptor::Passphrase(_) => Err(AppError::RuntimeError(
            "Passphrase based decryption not supported".to_string(),
        )),
    }
}
fn remove_comments(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn parse_public_keys(keys: &[String]) -> Result<Vec<Box<dyn Recipient + Send>>, AppError> {
    let recipients: Vec<_> = keys
        .iter()
        .map(|key| {
            age::x25519::Recipient::from_str(key)
                .map(|id| Box::new(id) as Box<dyn Recipient + Send>)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(recipients)
}

pub fn parse_private_keys(keys: &[String]) -> Result<Vec<Arc<dyn Identity>>, AppError> {
    let identities: Vec<_> = keys
        .iter()
        .map(|key| {
            age::x25519::Identity::from_str(&remove_comments(key))
                .map(|id| Arc::new(id) as Arc<dyn Identity>)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(identities)
}

pub fn get_public_key_for_private_key(private_key: &str) -> Result<String, AppError> {
    let identity = age::x25519::Identity::from_str(&remove_comments(private_key))?;
    Ok(identity.to_public().to_string())
}

pub fn gen_key() -> GeneratedKey {
    let identity = age::x25519::Identity::generate();
    return GeneratedKey {
        public_key: identity.to_public().to_string(),
        private_key: identity.to_string().expose_secret().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use age::secrecy::ExposeSecret;

    #[test]
    fn test_private_parse_keys() -> Result<(), String> {
        let identity = age::x25519::Identity::generate();
        let key_as_string: String = format!(
            "# created: 2023-07-12T12:16:23+02:00\n{}\n",
            identity.to_string().expose_secret().to_owned()
        );
        let parsed_identities = parse_private_keys(&[key_as_string])?;
        assert_eq!(parsed_identities.len(), 1);
        Ok(())
    }

    #[test]
    fn test_parse_public_keys() -> Result<(), String> {
        let key = "age1hwuyjjjvljra6j00vynkgaxap7zlh4fmadj09m4jn6t9t0nveyds6mt6zf";
        parse_public_keys(&[key.to_owned()])?;
        Ok(())
    }

    #[test]
    fn test_en_and_decrypt() -> Result<(), String> {
        let identity = age::x25519::Identity::generate();
        let encrypted = encrypt_message(vec![Box::new(identity.to_public())], "fkbr")?;
        let decrypted = decrypt_message(vec![Arc::new(identity)], &encrypted)?;
        assert_eq!(decrypted, "fkbr");

        Ok(())
    }

    #[test]
    fn test_gen_key() -> Result<(), String> {
        let generated_key = gen_key();
        parse_private_keys(&[generated_key.private_key])?;
        parse_public_keys(&[generated_key.public_key])?;
        Ok(())
    }
}
