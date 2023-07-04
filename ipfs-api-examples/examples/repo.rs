// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api_examples::ipfs_api::{response, IpfsApi, IpfsClient};
use futures::TryStreamExt;

fn print_stat(stat: response::RepoStatResponse) {
    eprintln!("  num objects     : {}", stat.num_objects);
    eprintln!("  size     : {}", stat.repo_size);
    eprintln!("  path     : {}", stat.repo_path);
    eprintln!("  version    : {}", stat.version);
    eprintln!();
}

// Creates an Ipfs client, and makes some calls to the Repo Api.
//
#[ipfs_api_examples::main]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    eprintln!("getting repo/stat...");
    eprintln!();

    match client.repo_stat().await {
        Ok(s) => print_stat(s),
        Err(err) => {
            eprintln!("error getting /repo/stat: {}", err);
            return;
        }
    }

    eprintln!("getting repo/version...");
    eprintln!();

    match client.repo_version().await {
        Ok(v) => println!("version: {}", v.version),
        Err(err) => {
            eprintln!("error getting /repo/version: {}", err);
            return;
        }
    }

    eprintln!("performing garbage collection...");
    eprintln!();

    let mut res = client.repo_gc().await;
    while let Some(n) = res.try_next().await.expect("stream error") {
        let val = n.key.get("/").expect("no value");
        println!("Garbage collection deletes: {}", val);
    }

    eprintln!("Fetching /repo/ls...");
    eprintln!();

    let mut res = client.repo_ls().await;
    while let Some(n) = res.try_next().await.expect("stream error") {
        println!("In repo: {:?}", n);
    }

    eprintln!("Calling /repo/verify...");
    eprintln!();

    let mut res = client.repo_verify().await;
    while let Some(n) = res.try_next().await.expect("stream error") {
        if n.msg.len() > 0 {
            println!("Message: {:?}", n.msg);
        }
    }

    eprintln!("done!");
}
