/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::ops::Deref;

use options::ProxyProtocol;
use reqwest::{NoProxy, Proxy};

pub mod error;
pub mod options;

#[derive(Clone)]
pub struct Client(reqwest::Client);

impl Client {
    pub fn new(opts: options::ClientOptions) -> Result<Self, error::ClientError> {
        let mut b = reqwest::ClientBuilder::new();
        let proxy_option = opts.proxy;
        b = apply_proxy(b, proxy_option)?;
        let cli = b.build()?;
        Ok(cli.into())
    }

    pub fn inner(&self) -> reqwest::Client {
        self.0.clone()
    }
}

fn apply_proxy(
    mut b: reqwest::ClientBuilder,
    proxy_option: Option<Vec<options::ProxyItem>>,
) -> Result<reqwest::ClientBuilder, error::ClientError> {
    if let Some(proxies) = proxy_option {
        if proxies.is_empty() {
            return Ok(b.no_proxy());
        } else {
            for p in proxies {
                let mut proxy = match p.protocol {
                    ProxyProtocol::All => Proxy::all(p.scheme),
                    ProxyProtocol::Http => Proxy::http(p.scheme),
                    ProxyProtocol::Https => Proxy::https(p.scheme),
                }?;
                // TODO: `NoProxy` fields are private
                let no_proxy = NoProxy::default();
                proxy = proxy.no_proxy(Some(no_proxy));
                b = b.proxy(proxy);
            }
        }
    }
    Ok(b)
}

impl From<reqwest::Client> for Client {
    fn from(val: reqwest::Client) -> Self {
        Client(val)
    }
}

impl From<Client> for reqwest::Client {
    fn from(val: Client) -> Self {
        val.0
    }
}

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod test {
    #[test]
    fn test_into() {
        use super::{Client, options::ClientOptions};
        let opts = ClientOptions { proxy: None };
        let client = Client::new(opts).unwrap();
        let _: reqwest::Client = client.into();
    }
}
