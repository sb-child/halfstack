/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Clone)]
pub struct AcmeOptions {}

#[derive(Deserialize, Clone)]
pub struct AccountOptions {
    pub endpoint: Url,
    pub account_type: AccountType,
}

#[derive(Deserialize, Clone)]
pub enum AccountType {
    NewAccount(NewAccount),
    AccountFromKey(AccountFromKey),
}

#[derive(Deserialize, Clone)]
pub struct NewAccount {
    pub contacts: Vec<String>,
    pub agree_tos: bool,
}

#[derive(Deserialize, Clone)]
pub struct AccountFromKey {
    pub key: String,
}
