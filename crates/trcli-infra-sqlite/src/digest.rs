//! The hash function that chains the audit trail: SHA-256 (FR-048).
//!
//! A hash function must not be hand-written, which is why this adapter, and not the
//! domain, depends on a cryptography crate.

use sha2::{Digest as _, Sha256};
use trcli_application::ports::audit::AuditDigest;
use trcli_domain::governance::audit::Digest;

/// SHA-256.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sha256Digest;

impl Sha256Digest {
    /// The SHA-256 digest of some bytes.
    pub fn of(bytes: &[u8]) -> Digest {
        Sha256::digest(bytes).into()
    }
}

impl AuditDigest for Sha256Digest {
    fn digest(&self, bytes: &[u8]) -> Digest {
        Self::of(bytes)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the digest.

    use trcli_domain::governance::audit::to_hex;

    use super::Sha256Digest;

    #[test]
    fn it_is_sha_256() {
        // The well-known digest of the empty input.
        assert_eq!(
            to_hex(&Sha256Digest::of(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
