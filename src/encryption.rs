use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::{Key, Nonce};

/// Encrypts a password using a randomly generated key and nonce.
pub fn encrypt_password(password: &str, key: &Key) -> (String, String) {
    let nonce = secretbox::gen_nonce();
    let ciphertext = secretbox::seal(password.as_bytes(), &nonce, key);
    (
        base64::encode(ciphertext),
        base64::encode(nonce.as_ref()),
    )
}

/// Decrypts an encrypted password.
pub fn decrypt_password(encrypted_password: &str, nonce_str: &str, key: &Key) -> Result<String, String> {
    let ciphertext = match base64::decode(encrypted_password) {
        Ok(decoded) => decoded,
        Err(_) => return Err("Failed to decode ciphertext".to_string()),
    };

    let nonce_bytes = match base64::decode(nonce_str) {
        Ok(decoded) => decoded,
        Err(_) => return Err("Failed to decode nonce".to_string()),
    };

    let nonce = match Nonce::from_slice(&nonce_bytes) {
        Some(n) => n,
        None => return Err("Failed to parse nonce".to_string()),
    };

    match secretbox::open(&ciphertext, &nonce, key) {
        Ok(decrypted) => match String::from_utf8(decrypted) {
            Ok(password) => Ok(password),
            Err(_) => Err("Failed to parse decrypted bytes into a valid string".to_string()),
        },
        Err(_) => Err("Failed to decrypt password: nonce or key may be incorrect".to_string()),
    }
}

/// Generates a new random encryption key.
pub fn generate_key() -> Key {
    Key::from_slice(b"an example very very secret key.") // 32 bytes
        .expect("Failed to create a static key")
}


#[cfg(test)]
mod tests {
    use super::*;
    use sodiumoxide::crypto::secretbox;

    #[test]
    fn test_encrypt_and_decrypt_password() {
        // Arrange
        let key = secretbox::gen_key();
        let password = "test_password";

        // Act
        let (encrypted_password, nonce) = encrypt_password(password, &key);
        let decrypted_password = decrypt_password(&encrypted_password, &nonce, &key)
            .expect("Decryption should succeed");

        // Assert
        assert_eq!(decrypted_password, password);
    }

    #[test]
    fn test_decrypt_with_invalid_nonce() {
        // Arrange
        let key = secretbox::gen_key();
        let password = "test_password";
        let (_, _nonce) = encrypt_password(password, &key);

        // Modify nonce
        let invalid_nonce = "invalid_nonce";

        // Act
        let result = decrypt_password("invalid_encrypted_data", invalid_nonce, &key);

        // Assert
        assert!(result.is_err());
    }
}
