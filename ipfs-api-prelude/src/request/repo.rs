// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

pub struct RepoVerify;

impl_skip_serialize!(RepoVerify);

impl ApiRequest for RepoVerify {
    const PATH: &'static str = "/repo/verify";
}

#[derive(Serialize)]
pub struct RepoGc {
    pub stream_errors: Option<bool>,
    pub quiet: Option<bool>,
    pub silent: Option<bool>,
}

impl ApiRequest for RepoGc {
    const PATH: &'static str = "/repo/gc";
}

#[derive(Serialize)]
pub struct RepoStat {
    pub size_only: Option<bool>,
    pub human: Option<bool>,
}

impl ApiRequest for RepoStat {
    const PATH: &'static str = "/repo/stat";
}

#[derive(Serialize)]
pub struct RepoVersion {
    pub quiet: Option<bool>,
}

impl ApiRequest for RepoVersion {
    const PATH: &'static str = "/repo/version";
}

pub struct RepoLs;

impl_skip_serialize!(RepoLs);

impl ApiRequest for RepoLs {
    const PATH: &'static str = "/repo/ls";
}
