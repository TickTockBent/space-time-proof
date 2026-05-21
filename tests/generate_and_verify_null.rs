//! Integration test: full init -> prove -> verify roundtrip with NullPow.
//!
//! This is the non-adversarial path — no RandomX, no cmake dependency.
//! Proves that the space proof is sound independent of the PoW mechanism,
//! and that tampering with proof indices is still rejected.

use std::{borrow::Cow, sync::atomic::AtomicBool};

use space_time_proof::{
    compression::{compress_indices, decompress_indexes, required_bits},
    config::{InitConfig, ScryptParams},
    initialize::{CpuInitializer, Initialize},
    metadata::ProofMetadata,
    pow::{NullPowProver, NullPowVerifier},
    prove::{self, generate_proof, Proof},
    verification::{Error, Mode, Verifier},
};
use tempfile::tempdir;

#[test]
fn test_generate_and_verify_with_null_pow() {
    let challenge = b"hello world, challenge me!!!!!!!";
    let datadir = tempdir().unwrap();

    let cfg = space_time_proof::config::ProofConfig {
        k1: 23,
        k2: 32,
        pow_difficulty: [0xFF; 32],
    };
    let init_cfg = InitConfig {
        min_num_units: 1,
        max_num_units: 1000,
        labels_per_unit: 256 * 16,
        scrypt: ScryptParams::new(2, 1, 1),
    };

    let metadata = CpuInitializer::new(init_cfg.scrypt)
        .initialize(
            datadir.path(),
            &[77; 32],
            &[0u8; 32],
            init_cfg.labels_per_unit,
            31,
            1000,
            None,
        )
        .unwrap();

    let stop = AtomicBool::new(false);
    let reporter = prove::NoopProgressReporter {};
    let pow_prover = NullPowProver;
    let proof = generate_proof(
        datadir.path(),
        challenge,
        cfg,
        32,
        space_time_proof::config::Cores::Any(1),
        stop,
        reporter,
        &pow_prover,
    )
    .unwrap();

    let metadata = ProofMetadata::new(metadata, *challenge);
    let verifier = Verifier::new(Box::new(NullPowVerifier));
    verifier
        .verify(&proof, &metadata, &cfg, &init_cfg, Mode::All)
        .expect("proof should be valid under NullPow");

    // Tampering with an index must still be rejected — the space proof
    // is sound independent of the PoW mechanism.
    let bits = required_bits(metadata.num_units as u64 * init_cfg.labels_per_unit);
    let mut indices = decompress_indexes(&proof.indices, bits).collect::<Vec<_>>();
    indices[7] ^= u64::MAX;
    let invalid_proof = Proof {
        indices: Cow::Owned(compress_indices(&indices, bits)),
        ..proof
    };
    let result = verifier.verify(&invalid_proof, &metadata, &cfg, &init_cfg, Mode::All);
    assert!(
        matches!(result, Err(Error::InvalidMsb { index_id, .. }) if index_id == 7),
        "tampered proof must be rejected"
    );
}
