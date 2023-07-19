use std::str::FromStr;
use std::sync::Arc;

use age::armor::ArmoredReader;
use age::Decryptor;
use age::Identity;
use std::io::{Cursor, Read};

pub fn decrypt_message(
    identities: Vec<Arc<dyn Identity>>,
    message: &str,
) -> Result<String, String> {
    let input = Cursor::new(message);

    match Decryptor::new(ArmoredReader::new(input)).map_err(|e| e.to_string())? {
        Decryptor::Recipients(d) => {
            let mut reader = d
                .decrypt(identities.iter().map(|arc| arc.as_ref()))
                .map_err(|e| e.to_string())?;
            let mut buffer = String::new();
            reader
                .read_to_string(&mut buffer)
                .map_err(|e| e.to_string())?;
            return Ok(buffer);
        }
        Decryptor::Passphrase(_) => {
            return Err("Passphrase based decryption not supported".to_string());
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

pub fn parse_private_keys(keys: Vec<String>) -> Result<Vec<Arc<dyn Identity>>, String> {
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
}
