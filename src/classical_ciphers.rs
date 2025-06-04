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
