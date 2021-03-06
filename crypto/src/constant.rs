// Copyright 2020 WeDPR Lab Project Authors. Licensed under Apache-2.0.

//! Shared constants.

#![allow(deprecated)]
use curve25519_dalek::{
    constants::{RISTRETTO_BASEPOINT_COMPRESSED, RISTRETTO_BASEPOINT_POINT},
    ristretto::RistrettoPoint,
};
extern crate secp256k1;
use crate::{
    ecies::secp256k1::WedprSecp256k1Ecies, hash::keccak256::WedprKeccak256,
    signature::secp256k1::WedprSecp256k1Recover,
};
use secp256k1::{All, Secp256k1, VerifyOnly};
use sha3::Sha3_512;
use wedpr_utils::coder::base_x::WedprBase64;

lazy_static! {
    /// A base point used by various crypto algorithms.
    pub static ref BASEPOINT_G1: RistrettoPoint = RISTRETTO_BASEPOINT_POINT;
    /// Another base point used by various crypto algorithms.
    pub static ref BASEPOINT_G2: RistrettoPoint =
        RistrettoPoint::hash_from_bytes::<Sha3_512>(
            RISTRETTO_BASEPOINT_COMPRESSED.as_bytes()
        );

    /// Shared secp256k1 instance initialized for verification function only.
    pub static ref SECP256K1_VERIFY: Secp256k1<VerifyOnly> = Secp256k1::verification_only();
    /// Shared secp256k1 instance initialized for all functions.
    pub static ref SECP256K1_ALL: Secp256k1<All> = Secp256k1::new();

    /// Shared coder algorithm reference for quick implementation replacement.
    /// Other code should use this reference, and not directly use a specific implementation.
    // You may change WedprBase64 to other coder implementation such as WedprHex.
    pub static ref CODER: WedprBase64 = WedprBase64::default();

    /// Shared signature algorithm reference for quick implementation replacement.
    /// Other code should use this reference, and not directly use a specific implementation.
    pub static ref SIGNATURE: WedprSecp256k1Recover =
        WedprSecp256k1Recover::default();

    /// Shared hash algorithm reference for quick implementation replacement.
    /// Other code should use this reference, and not directly use a specific implementation.
    pub static ref HASH: WedprKeccak256 = WedprKeccak256::default();

    /// Shared ECIES algorithm reference for quick implementation replacement.
    /// Other code should use this reference, and not directly use a specific implementation.
    pub static ref ECIES: WedprSecp256k1Ecies = WedprSecp256k1Ecies::default();
}

/// Serialized data size of a point.
pub const RISTRETTO_POINT_SIZE_IN_BYTES: usize = 32;

/// Constants only used by tests.
pub mod tests {
    // Test key pair for secp256k1 algorithms.
    pub const SECP256K1_TEST_SECRET_KEY: &str =
        "EMGwfgpqDQVUsVO7jxUniSNYxTPjGcbbf6eikaAIKog=";
    pub const SECP256K1_TEST_PUBLIC_KEY: &str =
        "ApBrmk0PHxPp4ElvnhlmwEiAklVdDbX+MicqML591eC2";

    // Test string for a base64 encoded message.
    pub const BASE64_ENCODED_TEST_MESSAGE: &str =
        "g6sLGLyLvnkOSvBcQdKNSPx8m4XmAogueNMmE6V0Ico=";
}
