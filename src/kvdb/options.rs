/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct KvDbOptions {
    pub path: PathBuf,
    #[serde(default = "t")]
    pub create_if_missing: bool,
}

fn t() -> bool {
    true
}
