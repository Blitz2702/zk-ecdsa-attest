# ZK-ECDSA Attestation: Privacy-Preserving Signature Proofs

> **A Rust implementation of a Non-Interactive Zero-Knowledge (NIZK) protocol for proving possession of valid ECDSA signatures without revealing the signature itself.**

![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-Research%20Prototype-yellow.svg)

## 📖 Overview

This library provides a Zero-Knowledge wrapper around standard ECDSA signatures (Secp256k1). It addresses a core problem in Remote Attestation: **Identity Privacy**.

In standard attestation (e.g., TPM, SGX), a device presents a signature $(r, s)$ to prove its state. However, this signature acts as a static identifier, allowing the verifier to track the device across sessions.

**This project solves that by proving knowledge of the signature in Zero-Knowledge.** The Prover convinces the Verifier that "I possess a valid signature" without ever revealing the signature scalar $s$.

## 📐 The Cryptography

To enable Zero-Knowledge proofs for ECDSA, we utilize a **Linearization Technique** to transform the non-linear modular inverse equation into a form compatible with Schnorr-style Sigma protocols.

### 1. The Transformation
Standard ECDSA verification checks:
$$R = s^{-1}(z \cdot G + r \cdot Q)$$

We linearize this by multiplying by $s$:
$$s \cdot R = z \cdot G + r \cdot Q$$

### 2. The ZK Relation
The Prover proves knowledge of the secret witness $s$ satisfying the relation:
$$s \cdot \text{Base} = \text{Target}$$

Where:
* **Secret:** $s$ (The hidden signature scalar)
* **Base:** $R$ (The nonce point from the signature)
* **Target:** $z \cdot G + r \cdot Q$ (Computed publicly by the Verifier)

### 3. Non-Interactivity (NIZK)
We employ the **Strong Fiat-Shamir Heuristic** to transform the interactive Sigma protocol into a non-interactive proof. The challenge $c$ is derived via a cryptographic hash of the transcript:
$$c = \text{SHA256}(\text{DomainSeparator} || T || R || Q || z)$$

---

## 🚀 Features

* **Secp256k1 Support:** Built on top of the `k256` crate (standard for Bitcoin/Ethereum).
* **Modular Architecture:** Strict separation of `Prover`, `Verifier`, and `Transcript` logic.
* **Replay Protection:** Transcript hashing includes public inputs and domain separation tags.
* **Type Safety:** Leverages Rust's type system to distinguish between `Scalar`, `ProjectivePoint`, and `Witness` types.
* **Comprehensive Testing:** Includes integration tests verifying end-to-end flows.

---

## 📂 Project Structure

```text
zk-ecdsa-attest/
├── Cargo.toml            # Dependencies (k256, sha2, rand)
├── src/
│   ├── lib.rs            # Library entry point & module exports
│   ├── transcript.rs     # Shared Fiat-Shamir hashing logic
│   ├── prover.rs         # Prover logic (Commit -> Respond)
│   ├── verifier.rs       # Verifier logic (Reconstruct -> Check)
│   └── main.rs           # CLI Demo (The "Story")
└── tests/
    └── integration_test.rs # End-to-End integration tests
