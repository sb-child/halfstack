/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unsupported curve")]
    UnsupportedCurve,
    #[error("missing required field")]
    MissingField,
    #[error("invalid key parameters")]
    InvalidKeyParameters,
    #[error("unsupported algorithm")]
    UnsupportedAlgorithm,
}

#[derive(Debug, thiserror::Error)]
pub enum ComposeError {
    #[error("key conversion failed")]
    KeyConversionFailed,
}

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("hash error: {0}")]
    HashError(#[from] std::io::Error),

    #[error("signature error: {0}")]
    SignatureError(#[from] p256::ecdsa::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("hash error: {0}")]
    HashError(#[from] std::io::Error),

    #[error("verify error: {0}")]
    VerifyError(#[from] p256::ecdsa::Error),
}
