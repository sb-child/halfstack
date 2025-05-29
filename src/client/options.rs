/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::net::IpAddr;

use ipnet::IpNet;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ClientOptions {
    /// - `None`: 使用系统代理
    /// - `Some([])`: 不使用代理
    /// - `Some([proxy, ...])`: 使用指定代理
    pub proxy: Option<Vec<ProxyItem>>,
}

#[derive(Deserialize, Clone)]
pub struct ProxyItem {
    /// - `http://`: HTTP Proxy
    /// - `https://`: HTTPS Proxy
    /// - `socks4://`: SOCKS4 Proxy
    /// - `socks4a://`: SOCKS4 Proxy + DNS
    /// - `socks5://`: SOCKS5 Proxy
    /// - `socks5h://`: SOCKS5 Proxy + DNS
    pub scheme: url::Url,
    /// - `Http`: Only HTTP requests
    /// - `Https`: Only HTTPS requests
    /// - `All`: Both
    #[serde(default)]
    pub protocol: ProxyProtocol,
    /// IP addresses bypass this proxy
    #[serde(default = "Vec::new")]
    pub blacklist_ip_addr: Vec<IpAddr>,
    /// IP CIDRs bypass this proxy
    #[serde(default = "Vec::new")]
    pub blacklist_ip_range: Vec<IpNet>,
    /// Domain matchers bypass this proxy
    #[serde(default = "Vec::new")]
    pub blacklist_domain_matcher: Vec<String>,
}

#[derive(Deserialize, Clone, Default)]
pub enum ProxyProtocol {
    #[default]
    All,
    Http,
    Https,
}
