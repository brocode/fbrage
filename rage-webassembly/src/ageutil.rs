use std::str::FromStr;
use std::sync::Arc;

use crate::error::AppError;
use age::armor::ArmoredReader;
use age::armor::ArmoredWriter;
use age::armor::Format;
use age::Decryptor;
use age::Identity;
use age::Recipient;
use std::io::Write;
use std::io::{Cursor, Read};

pub fn encrypt_message(
    message: &str,
    recipients: Vec<Box<dyn Recipient + Send>>,
) -> Result<String, AppError> {
    let mut output_vec = Vec::new();
    let mut output = ArmoredWriter::wrap_output(&mut output_vec, Format::AsciiArmor)?;

    let encryptor =
        age::Encryptor::with_recipients(recipients).ok_or("Cannot create recipients")?;

    let mut writer = encryptor.wrap_output(&mut output)?;

    writer.write_all(message.as_bytes()).unwrap();
    writer.finish()?;
    output.finish()?;
    return Ok(String::from_utf8(output_vec)?);
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
            return Ok(buffer);
        }
        Decryptor::Passphrase(_) => {
            return Err(AppError::RuntimeError(
                "Passphrase based decryption not supported".to_string(),
            ));
        }
    }
}
fn remove_comments(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn parse_private_keys(keys: Vec<String>) -> Result<Vec<Arc<dyn Identity>>, AppError> {
    let identities: Vec<_> = keys
        .iter()
        .map(|key| {
            age::x25519::Identity::from_str(&remove_comments(key))
                .map(|id| Arc::new(id) as Arc<dyn Identity>)
        })
        .collect::<Result<Vec<_>, _>>()?;
    return Ok(identities);
}

#[cfg(test)]
mod tests {
    use super::*;
    use age::secrecy::ExposeSecret;

    #[test]
    fn test_parse_keys() -> Result<(), String> {
        let identity = age::x25519::Identity::generate();
        let key_as_string: String = format!(
            "# created: 2023-07-12T12:16:23+02:00\n{}\n",
            identity.to_string().expose_secret().to_owned()
        );
        let parsed_identities = parse_private_keys(vec![key_as_string])?;
        assert_eq!(parsed_identities.len(), 1);
        return Ok(());
    }

    #[test]
    fn test_en_and_decrypt() -> Result<(), String> {
        let identity = age::x25519::Identity::generate();
        let encrypted = encrypt_message("fkbr", vec![Box::new(identity.to_public())])?;
        let decrypted = decrypt_message(vec![Arc::new(identity)], &encrypted)?;
        assert_eq!(decrypted, "fkbr");

        return Ok(());
    }
}
