//! Shared exact-byte hashing for model revisions and Explorer wire artifacts.

use sha2::{Digest, Sha256};

/// Return the SHA-256 digest of exactly `bytes` as 64 lowercase hexadecimal
/// characters. Input framing and canonicalization belong to the caller.
pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::sha256_hex;

    #[test]
    fn exact_bytes_match_independent_known_answers() {
        // Empty/abc are standard SHA-256 vectors; the remaining fixed digests
        // were independently calculated with Python hashlib.
        let vectors: &[(&[u8], &str)] = &[
            (
                b"",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                b"abc",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                &[0, 255, 128, 1, 0, 254],
                "d28d28e779235058550fa5e544e8320bfb604b4814e2781f0ac9618b6cd3a800",
            ),
            (
                "Café 測定.".as_bytes(),
                "3a8cc65951dbb5d7fac1fc61f4451930799430c5c2198cebc757690fc333871d",
            ),
            (
                b"a b",
                "c8687a08aa5d6ed2044328fa6a697ab8e96dc34291e8c2034ae8c38e6fcc6d65",
            ),
            (
                b"ab",
                "fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603",
            ),
            (
                b"abc\n",
                "edeaaff3f1774ad2888673770c6d64097e391bc362d7d6fb34982ddf0efd18cb",
            ),
            (
                b"abc\r\n",
                "552bab6864c7a7b69a502ed1854b9245c0e1a30f008aaa0b281da62585fdb025",
            ),
        ];
        for (input, expected) in vectors {
            let original = input.to_vec();
            let actual = sha256_hex(input);
            assert_eq!(actual, *expected);
            assert_eq!(actual.len(), 64);
            assert!(actual
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
            assert_eq!(*input, original);
        }
    }
}
