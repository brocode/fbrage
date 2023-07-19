use std::str::FromStr;

use age::x25519::Identity;

fn remove_comments(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn parse_private_keys(keys: Vec<String>) -> Result<Vec<Identity>, String> {
    let identities: Vec<_> = keys
        .iter()
        .map(|key| Identity::from_str(&remove_comments(key)))
        .collect::<Result<Vec<_>, _>>()?;
    return Ok(identities);
}

#[cfg(test)]
mod tests {
    use super::*;
    use age::secrecy::ExposeSecret;

    #[test]
    fn test_parse_keys() -> Result<(), String> {
        let identity = Identity::generate();
        let key_as_string: String = format!(
            "# created: 2023-07-12T12:16:23+02:00\n{}\n",
            identity.to_string().expose_secret().to_owned()
        );
        let parsed_identities = parse_private_keys(vec![key_as_string])?;
        assert_eq!(parsed_identities.len(), 1);
        return Ok(());
    }
}
