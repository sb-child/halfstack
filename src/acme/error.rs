/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::client;

#[derive(Debug, thiserror::Error)]
pub enum AcmeError {
    #[error("Options Error")]
    OptionsError(),

    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}
