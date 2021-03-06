use crate::error::ClientError;
use crypto_exports::bellman::{pairing::ff::PrimeField, PrimeFieldRepr};
use crypto_exports::franklin_crypto::alt_babyjubjub::fs::FsRepr;
use models::node::{priv_key_from_fs, Fs, PrivateKey};
use sha2::{Digest, Sha256};

// Public re-exports.
pub use models::node::{
    closest_packable_fee_amount, closest_packable_token_amount, is_fee_amount_packable,
    is_token_amount_packable,
};

/// Generates a new `PrivateKey` from seed using a deterministic algorithm:
/// seed is hashed via `sha256` hash, and the output treated as a `PrivateKey`.
/// If the obtained value doesn't have a correct value to be a `PrivateKey`, hashing operation is applied
/// repeatedly to the previous output, until the value can be interpreted as a `PrivateKey`.
pub fn private_key_from_seed(seed: &[u8]) -> Result<PrivateKey, ClientError> {
    if seed.len() < 32 {
        return Err(ClientError::SeedTooShort);
    }

    let sha256_bytes = |input: &[u8]| -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(input);
        hasher.result().to_vec()
    };

    let mut effective_seed = sha256_bytes(seed);

    loop {
        let raw_priv_key = sha256_bytes(&effective_seed);
        let mut fs_repr = FsRepr::default();
        fs_repr
            .read_be(&raw_priv_key[..])
            .expect("failed to read raw_priv_key");
        match Fs::from_repr(fs_repr) {
            Ok(fs) => return Ok(priv_key_from_fs(fs)),
            Err(_) => {
                effective_seed = raw_priv_key;
            }
        }
    }
}
