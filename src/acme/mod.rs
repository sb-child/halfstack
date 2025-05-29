/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{client::Client, kvdb::KvDb};

pub mod error;
pub mod options;
pub mod requirements;

pub struct Acme {
    client: Client,
    storage: KvDb,
}

impl Acme {
    pub fn new(
        _opts: options::AcmeOptions,
        reqs: requirements::AcmeRequirements,
    ) -> Result<Self, error::AcmeError> {
        Ok(Self {
            client: reqs.client,
            storage: reqs.storage,
        })
    }

    pub async fn get_account(
        &self,
        opts: options::AccountOptions,
    ) -> Result<AccountHandle, error::AcmeError> {
        let sign = tokio::task::spawn_blocking(|| {
            // calculate
            0
        })
        .await?;
        let req = self.client.get("https://example.nya/xxx").build()?;
        let resp = self.client.execute(req).await?;
        let _ = resp.status();
        Ok(AccountHandle(0))
    }
    pub fn create_order() {}
    pub fn verify() {}
    pub fn download_certificate() {}
}

#[derive(Clone, Copy)]
pub struct AccountHandle(i64);
