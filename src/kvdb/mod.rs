/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{ops::Deref, sync::Arc};

pub mod error;
pub mod options;

#[derive(Clone)]
pub struct KvDb(Arc<rocksdb::DB>);

impl KvDb {
    pub fn new(opts: options::KvDbOptions) -> Result<Self, error::KvDbError> {
        let mut database_options = rocksdb::Options::default();
        database_options.create_if_missing(opts.create_if_missing);
        let db = rocksdb::DB::open(&database_options, opts.path)?;
        Ok(Self(Arc::new(db)))
    }

    pub fn inner(&self) -> Arc<rocksdb::DB> {
        self.0.clone()
    }
}

impl Deref for KvDb {
    type Target = Arc<rocksdb::DB>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
