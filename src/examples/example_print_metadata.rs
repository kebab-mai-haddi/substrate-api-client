/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

///! Very simple example that shows how to pretty print the metadata. Has proven to be a helpful
///! debugging tool.

#[macro_use]
extern crate clap;

use clap::App;

use substrate_api_client::node_metadata;
use substrate_api_client::Api;

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let api = Api::new(format!("ws://{}", url));

    let meta = api.get_metadata();
    println!(
        "Metadata:\n {}",
        node_metadata::pretty_format(&meta).unwrap()
    );
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
