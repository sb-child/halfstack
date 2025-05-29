/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unsupported curve type")]
    UnsupportedCurve,
    #[error("invalid JWK format")]
    InvalidJwkFormat,
    #[error("missing required field")]
    MissingField,
    #[error("invalid key parameters")]
    InvalidKeyParameters,
}

#[derive(Debug, thiserror::Error)]
pub enum ComposeError {
    #[error("key conversion failed")]
    KeyConversionFailed,
    #[error("invalid key parameters")]
    InvalidKeyParameters,
}
