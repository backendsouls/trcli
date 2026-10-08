//! Fakes of the audit trail's surroundings: a stand-in digest and an in-memory head.

use std::cell::RefCell;

use trcli_domain::governance::audit::{AuditHead, Digest};

use crate::ports::audit::{AuditDigest, AuditHeadStore};
use crate::ports::unit_of_work::StoreError;

/// A digest that is not cryptographic but changes whenever its input does, which is all
/// the chain's tests need. The real one is SHA-256, in the SQLite adapter.
#[derive(Clone, Copy, Debug, Default)]
pub struct ToyDigest;

impl ToyDigest {
    /// Digests bytes.
    pub fn of(bytes: &[u8]) -> Digest {
        let mut digest = [0_u8; 32];
        let mut carry = 0x9E_u8;
        for (index, byte) in bytes.iter().enumerate() {
            let slot = index % 32;
            carry = carry.rotate_left(3) ^ byte.wrapping_add(index as u8);
            digest[slot] = digest[slot].wrapping_mul(31).wrapping_add(carry);
        }
        digest
    }
}

impl AuditDigest for ToyDigest {
    fn digest(&self, bytes: &[u8]) -> Digest {
        Self::of(bytes)
    }
}

/// A head kept in memory.
#[derive(Debug, Default)]
pub struct MemoryHead {
    /// The stored head.
    head: RefCell<Option<AuditHead>>,
    /// Whether writing fails, as on a read-only disk.
    read_only: bool,
}

impl MemoryHead {
    /// No head yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// A head that is already stored.
    pub fn holding(head: AuditHead) -> Self {
        Self {
            head: RefCell::new(Some(head)),
            read_only: false,
        }
    }

    /// The stored head.
    pub fn current(&self) -> Option<AuditHead> {
        *self.head.borrow()
    }
}

impl AuditHeadStore for MemoryHead {
    fn read_head(&self) -> Result<Option<AuditHead>, StoreError> {
        Ok(*self.head.borrow())
    }

    fn write_head(&self, head: &AuditHead) -> Result<(), StoreError> {
        if self.read_only {
            return Err(StoreError::ReadOnly("the head file".to_owned()));
        }
        *self.head.borrow_mut() = Some(*head);
        Ok(())
    }
}
