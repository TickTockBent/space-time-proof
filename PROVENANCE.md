# Provenance

This is `space-time-proof` 0.1.0, a hard fork of
[spacemeshos/post-rs](https://github.com/spacemeshos/post-rs) under the MIT
license.

This is **not** a continuation of the upstream project. The Spacemesh
operating company declared insolvency in May 2025, no maintainer is
accepting PRs upstream, and the upstream repository has been frozen since.
The fork was created in May 2026 to extract and preserve the PoST primitive
for reuse outside the original blockchain context.

The rename (`post` → `space-time-proof`) and version reset (`0.8.5` → `0.1.0`)
make the discontinuity legible to downstream consumers reading a Cargo.toml.
Lineage is documented here, not encoded in the version number.

## Fork point

Forked from `spacemeshos/post-rs` at commit
[`a2d155a2a1c13c59559737634262b67686e503b1`](https://github.com/spacemeshos/post-rs/commit/a2d155a2a1c13c59559737634262b67686e503b1)
(2025-05-20), the last commit before upstream became unmaintained.

## Dependency chain of custody

The two C-backed crypto dependencies — and their submodules — were forked
under the same `TickTockBent` namespace and rev-pinned to capture them
against any future deletion of the upstream repositories. The submodule
URLs in each Rust binding fork were re-pointed at the corresponding C
library fork, so cloning the dependency tree never touches a Spacemesh- or
external upstream- controlled repository.

| Dependency | Original upstream | Forked to | Pinned at | Date |
|---|---|---|---|---|
| `scrypt-jane-rs` (Rust binding) | `spacemeshos/scrypt-jane-rs` | `TickTockBent/scrypt-jane-rs` | `9e118d7` | 2026-05-19 (re-point commit on top of upstream `79634704` from 2023-06-06) |
| `scrypt-jane` (C library, submodule) | `spacemeshos/scrypt-jane` (downstream of `floodyberry/scrypt-jane`) | `TickTockBent/scrypt-jane` | `f09a907` | 2023-03-08 |
| `randomx-rs` (Rust binding) | `spacemeshos/randomx-rs` (downstream of `tari-project/randomx-rs`) | `TickTockBent/randomx-rs` | `5e4adca` (branch: `development`) | 2026-05-19 (re-point commit on top of upstream `d46bcd90` from 2023-11-20) |
| `RandomX` (C library, submodule) | `tevador/RandomX` (canonical upstream, also used by Monero) | `TickTockBent/RandomX` | `102f8acf` (tag: `v1.2.1`) | 2023-10-20 |

All four forks preserve full upstream git history. The only modification
in each Rust-binding fork is a single-line `.gitmodules` change to re-point
the C-library submodule URL at the corresponding TickTockBent fork.

## Modifications from upstream `post-rs`

- **Workspace strip.** Removed sub-crates `ffi`, `scrypt-ocl`, `initializer`,
  `profiler`, `service`, `certifier`, `k2pow-service`. Only the root crate
  remains; this is a single-crate repository now.
- **Dead-code removal.** Deleted `src/pow/service.rs` (K2 PoW remote-service
  client; no internal callers, only consumed by the deleted `service/`
  member and the deleted FFI). With this, `reqwest`, `tokio`, and `futures`
  could be dropped from dependencies.
- **Crate rename and version reset.** `post-rs` → `space-time-proof`; lib
  name `post` → `space_time_proof`; `0.8.5` → `0.1.0`. The `[lib]` table was
  dropped — cargo derives the library name from the package name by default.
- **PoW becomes optional.** RandomX-backed proof-of-work moved behind a
  `randomx` Cargo feature (default off). The `pow::Prover` and
  `pow::PowVerifier` traits stay unconditional in the core; the
  `pow::randomx` module is `#[cfg(feature = "randomx")]`.
- **`NullPowProver` and `NullPowVerifier` added.** New `src/pow/null.rs`
  provides the default PoW path: prover always returns `pow = 0`, verifier
  accepts any value. **SECURITY**: the pow value feeds the AES cipher key
  derivation `blake3(challenge || nonce_group || pow)`. With `pow = 0` the
  AES key is derivable without RandomX cost, so the Null path is safe only
  for non-adversarial scenarios where the prover has no incentive to fake
  storage. See `src/pow/null.rs` for the full security commentary.
- **`RANDOMX_CACHE_KEY` constant removed.** The hardcoded
  `b"spacemesh-randomx-cache-key"` was the last Spacemesh-specific value in
  the code path. `PoW::new(flags)` becomes `PoW::new(flags, cache_key: &[u8])`;
  callers enabling the `randomx` feature pick their own cache key.
- **`mockall` moved to `[dev-dependencies]`.** All `#[automock]` attributes
  on trait definitions (`Prover`, `PowVerifier`, `ProgressReporter`,
  `Initialize`) are now `#[cfg_attr(test, automock)]`. `use mockall::...`
  imports are `#[cfg(test)]`-gated.
- **`generate_proof()` signature.** Removed the `pow_flags: RandomXFlag`
  parameter; it was used only for one log line, and the caller already
  knows the flags it constructed its prover with. This is a breaking
  change captured by the version reset.
- **`prove_many` panic stub replaced.** The RandomX backend's
  `prove_many` method returned `panic!("not implemented")`; replaced with
  `Err(Error::Unsupported)`. A new `Error::Unsupported` variant was added
  to `pow::Error`.
- **Cargo profiles trimmed.** `[profile.release-clib]` and
  `[profile.dev-clib]` removed (FFI-specific, no longer relevant).

## License

MIT. The MIT terms of upstream `post-rs` are preserved.

Full dependency license chain:

- `space-time-proof`: MIT (this crate, inherited from upstream)
- `scrypt-jane-rs`: MIT
- `scrypt-jane` (C library): Public Domain / MIT
- `randomx-rs`: BSD-3-Clause
- `RandomX` (C library): BSD-3-Clause
- `aes` (RustCrypto): MIT / Apache-2.0
- `blake3`: CC0-1.0 / Apache-2.0
- All other crates.io dependencies: standard Rust ecosystem MIT / Apache-2.0

No GPL anywhere in the chain. Clean for commercial use.
