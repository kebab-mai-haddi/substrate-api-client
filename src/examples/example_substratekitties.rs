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

///! Very simple example that shows how to get some simple storage values.
use clap::{load_yaml, App};
use keyring::AccountKeyring;
use std::option::Option;
use substrate_api_client::{utils::hexstr_to_u256, Api};

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();
    let mut api = Api::new(format!("ws://{}", url));
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);

    // get some plain storage value
    // let file_key: Option<Vec<u8>> = Some("0x0000000000000000000000000000000000000000000000000000000000000001".as_bytes().to_vec());
    let file_hash = "0x0000000000000000000000000000000000000000000000000000000000000001";
    let result_str = api.get_file_storage("KittyStorage", "Value", file_hash).unwrap();
    println!("Result string is: {}", result_str);
    let result = hexstr_to_u256(result_str).unwrap();
    println!("The result is {}", result);

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
