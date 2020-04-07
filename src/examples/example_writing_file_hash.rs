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

//! This examples shows how to use the compose_extrinsic macro to create an extrinsic for any (custom)
//! module, whereas the desired module and call are supplied as a string.

use clap::{load_yaml, App};
use keyring::AccountKeyring;
use sp_core::crypto::Pair;
use substrate_api_client::{compose_extrinsic, utils::hexstr_to_hash, Api};
fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();
    let from = AccountKeyring::Alice.pair();
    let mut api = Api::new(format!("ws://{}", url)).set_signer(from);
    let file_hash =
        hexstr_to_hash("0x0000000000000000000000000000000000000000000000000000000000000002".to_string())
            .unwrap();
    println!("File hash has become: {:?}", file_hash);
    let xt = compose_extrinsic!(api.clone(), "Substratekitties", "set_value", file_hash);
    println!("[+] Composed Extrinsic:\n {:?}\n", xt);
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    let tx_hash = api.send_extrinsic(xt.hex_encode()).unwrap();
    println!("[+] Transaction got finalized. Hash: {:?}", tx_hash);
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