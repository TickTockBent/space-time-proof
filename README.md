# space-time-proof

A Rust library for **Proof of Space-Time (PoST)** proving and verification,
extracted from [spacemeshos/post-rs](https://github.com/spacemeshos/post-rs)
under the MIT license.

See [PROVENANCE.md](PROVENANCE.md) for the fork context, dependency chain
of custody, and full list of modifications.

## What it does

- Initialize PoST data (on CPU via [scrypt-jane](https://github.com/floodyberry/scrypt-jane))
- Generate proofs against a challenge
- Verify proofs (full, single-index, or random-subset modes)

The protocol is described at [`spacemeshos/protocol/post.md`](https://github.com/spacemeshos/protocol/blob/master/post.md).

## What's different from upstream

This is a **hard fork**, not a continuation:

- The crate is renamed (`post-rs` → `space-time-proof`) and versions
  restart at `0.1.0`.
- Proof-of-Work is now **optional**, behind a `randomx` Cargo feature.
  By default the library uses `NullPowProver` / `NullPowVerifier`, which
  are safe for non-adversarial use (storage proofs to a trusted verifier,
  custody attestation, possession proofs in federated systems).
- The Spacemesh-specific `RANDOMX_CACHE_KEY` constant is gone; callers
  enabling the `randomx` feature supply their own.
- Workspace members specific to the Spacemesh deployment (FFI, gRPC
  service, certifier, K2 PoW service, profiler, GPU OpenCL init) are
  removed. This is a single-crate library.

## Build matrix

| Features | Build deps | Use case |
|---|---|---|
| *(default)* | `clang` (for `scrypt-jane-sys` bindgen) | Non-adversarial storage / custody / possession proofs to a trusted verifier |
| `randomx` | `clang` + `cmake` (for the RandomX C library) | Adversarial scenarios (consensus, untrusted provers) where grinding must be cost-gated by proof-of-work |

**Security note** on the default (Null PoW) path: the PoW value is mixed
into the AES cipher key that gates label selection
(`blake3(challenge || nonce_group || pow)`). With `pow = 0`, an attacker
searching the `(nonce_group, label_indices)` space pays only AES + scrypt
per candidate, not RandomX. The space proof itself is still sound — you
must hold the labels — but the grinding cost on nonce iteration is
removed. Enable the `randomx` feature for adversarial use. See
[`src/pow/null.rs`](src/pow/null.rs) for the full security commentary.

## Quick usage (Null PoW)

```rust
use space_time_proof::{
    config::{InitConfig, ScryptParams},
    initialize::{CpuInitializer, Initialize},
    metadata::ProofMetadata,
    pow::{NullPowProver, NullPowVerifier},
    prove::{self, generate_proof},
    verification::{Mode, Verifier},
};

// Initialize PoST data
let metadata = CpuInitializer::new(scrypt_params)
    .initialize(datadir, &node_id, &commitment_atx_id, labels_per_unit, num_units, max_file_size, None)?;

// Prove
let proof = generate_proof(
    datadir,
    &challenge,
    proof_cfg,
    nonces_size,
    cores,
    AtomicBool::new(false),
    prove::NoopProgressReporter {},
    &NullPowProver,
)?;

// Verify
let verifier = Verifier::new(Box::new(NullPowVerifier));
verifier.verify(&proof, &ProofMetadata::new(metadata, challenge), &proof_cfg, &init_cfg, Mode::All)?;
```

For RandomX-backed proving, depend on this crate with `features = ["randomx"]`
and substitute `pow::randomx::PoW::new(flags, cache_key)` for the Null types.

## License

MIT. See [LICENSE](LICENSE) and [PROVENANCE.md](PROVENANCE.md) for the
full license chain across all transitive dependencies (no GPL).
