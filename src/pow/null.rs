//! Null proof-of-work implementation for non-adversarial use cases.
//!
//! # SECURITY
//!
//! [`NullPowProver`] always returns `pow = 0`. This value is mixed into the
//! AES cipher key derivation: `blake3(challenge || nonce_group || pow)`.
//! With pow fixed at 0, the AES key is derivable without any computational
//! cost beyond blake3.
//!
//! Concretely: an attacker searching the (nonce_group, label_indices)
//! space pays only AES + scrypt per candidate, not RandomX. The space
//! proof itself is still sound — you must hold the stored labels to find
//! K2 indices below difficulty — but the computational gate on nonce
//! iteration is removed.
//!
//! **Safe for:**
//! - Storage proofs to a trusted verifier
//! - Possession proofs in a federated system
//! - Any context where the prover has no incentive to fake storage
//!
//! **Not safe for:**
//! - Adversarial consensus (blockchain mining)
//! - Untrusted prover scenarios where grinding could substitute for storage
//!
//! For adversarial use, enable the `randomx` feature.

use std::ops::Range;

use super::{Error, PowVerifier, Prover};

/// PoW prover that always succeeds with `pow = 0`.
///
/// See [module-level docs](self) for security implications.
pub struct NullPowProver;

impl Prover for NullPowProver {
    fn prove(
        &self,
        _nonce_group: u8,
        _challenge: &[u8; 8],
        _difficulty: &[u8; 32],
        _miner_id: &[u8; 32],
    ) -> Result<u64, Error> {
        Ok(0)
    }

    fn prove_many(
        &self,
        nonce_groups: Range<u32>,
        _challenge: &[u8; 8],
        _difficulty: &[u8; 32],
        _miner_id: &[u8; 32],
    ) -> Result<Vec<(u32, u64)>, Error> {
        Ok(nonce_groups.map(|g| (g, 0u64)).collect())
    }

    fn par(&self) -> bool {
        false
    }
}

/// PoW verifier that accepts any value unconditionally.
///
/// The verification pipeline still calls `verify()` at the same call site —
/// the gate is present but always opens. Swapping in a real verifier later
/// requires no changes to call sites.
///
/// See [module-level docs](self) for security implications.
pub struct NullPowVerifier;

impl PowVerifier for NullPowVerifier {
    fn verify(
        &self,
        _pow: u64,
        _nonce_group: u8,
        _challenge: &[u8; 8],
        _difficulty: &[u8; 32],
        _miner_id: &[u8; 32],
    ) -> Result<(), Error> {
        Ok(())
    }
}
