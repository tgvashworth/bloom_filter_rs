extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

pub fn hash(v: &str) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(v);
    hasher.result_str()
}

pub fn hash_rounds(v: &str, rounds: usize) -> Vec<String> {
    if rounds <= 0 {
        vec![]
    } else {
        let mut hashed_vec = Vec::with_capacity(rounds);
        hashed_vec.push(hash(v));
        for i in 1..rounds {
            let hashed = hash(&hashed_vec[i - 1][..]);
            hashed_vec.push(hashed);
        }
        hashed_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_hash() {
        assert_eq!(
            hash("a"),
            "80084bf2fba02475726feb2cab2d8215eab14bc6bdd8bfb2c8151257032ecd8b"
        );
    }

    #[test]
    fn basic_hash_rounds() {
        assert_eq!(
            hash_rounds("a", 2),
            vec![
                "80084bf2fba02475726feb2cab2d8215eab14bc6bdd8bfb2c8151257032ecd8b",
                "9b6e081248e7136e3ff74d743de698f7eb97c2311f81ae680c278c5cfeb7ad18"
            ]
        );
    }
}
