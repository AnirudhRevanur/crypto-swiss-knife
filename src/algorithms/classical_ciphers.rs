use playfair_cipher::{cryptable::Cypher, playfair};

pub fn caesar_cipher_encrypt(plaintext: String, key: i32) -> String {
    fn shift_char(c: char, key: i32, base: char) -> char {
        let alphabet_length: u8 = 26;
        let a = base as u8;
        let k = ((key % alphabet_length as i32 + alphabet_length as i32) % alphabet_length as i32)
            as u8;
        (((c as u8 - a + k) % alphabet_length) + a) as char
    }

    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                shift_char(c, key, 'A')
            } else if c.is_ascii_lowercase() {
                shift_char(c, key, 'a')
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_cipher_decrypt(ciphertext: String, key: i32) -> String {
    fn shift_char(c: char, key: i32, base: char) -> char {
        let alphabet_length: u8 = 26;
        let a = base as u8;
        let k = ((key % alphabet_length as i32 + alphabet_length as i32) % alphabet_length as i32)
            as u8;
        (((c as u8 - a + k) % alphabet_length) + a) as char
    }

    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                shift_char(c, -key, 'A')
            } else if c.is_ascii_lowercase() {
                shift_char(c, -key, 'a')
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_cipher_encrypt(plaintext: String, key: String) -> String {
    fn shift_char(c: char, key_char: char, base: char) -> char {
        let alphabet_length: u8 = 26;
        let a = base as u8;
        let k = (key_char.to_ascii_lowercase() as u8 - b'a') % alphabet_length;
        (((c as u8 - a + k) % alphabet_length) + a) as char
    }

    let key_chars: Vec<char> = key.chars().collect();
    let key_len = key_chars.len();
    let mut key_index = 0;

    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let result = shift_char(c, key_chars[key_index % key_len], 'A');
                key_index += 1;
                result
            } else if c.is_ascii_lowercase() {
                let result = shift_char(c, key_chars[key_index % key_len], 'a');
                key_index += 1;
                result
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_cipher_decrypt(ciphertext: String, key: String) -> String {
    fn shift_char(c: char, key_char: char, base: char) -> char {
        let alphabet_length: u8 = 26;
        let a = base as u8;
        let k = (key_char.to_ascii_lowercase() as u8 - b'a') % alphabet_length;
        (((c as u8 - a + 26 - k) % alphabet_length) + a) as char
    }

    let key_chars: Vec<char> = key.chars().collect();
    let key_len = key_chars.len();
    let mut key_index = 0;

    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let result = shift_char(c, key_chars[key_index % key_len], 'A');
                key_index += 1;
                result
            } else if c.is_ascii_lowercase() {
                let result = shift_char(c, key_chars[key_index % key_len], 'a');
                key_index += 1;
                result
            } else {
                c
            }
        })
        .collect()
}

pub fn playfair_encrypt(plaintext: String, key: String) -> String {
    let playfair_key = playfair::PlayFairKey::new(key.as_str());
    playfair_key
        .encrypt(plaintext.as_str())
        .unwrap_or_else(|_| String::from("Something went wrong"))
}

pub fn playfair_decrypt(ciphertext: String, key: String) -> String {
    let playfair_key = playfair::PlayFairKey::new(key.as_str());
    playfair_key
        .decrypt(ciphertext.as_str())
        .unwrap_or_else(|_| String::from("Something went wrong"))
}
