//! Hashing utilities.

use std::collections::HashMap;

// from hashlib::sha512

/// Generate a SHA512 hash.
pub fn gen_sha512_hash(
    item: HashMap<String, Any>,
    hashcode: impl Iterator<Item = String>
) -> String {
    let hashed = hashcode.iter().map(|column|
        item.get(column).unwrap_or(&"".to_string())
    ).collect::<Vec<_>>().join("");
    let value = sha512(hashed.encode("utf-8"), usedforsecurity=False).hexdigest();
    format!("{value}")
}
