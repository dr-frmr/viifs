use kinode_process_lib::{await_message, call_init, eth, net, println, Address, Message, Request};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

/// From kns_indexer process
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct KnsState {
    chain_id: u64,
    // what contract this state pertains to
    contract_address: eth::Address,
    // namehash to human readable name
    names: std::collections::HashMap<String, String>,
    // human readable name to most recent on-chain routing information as json
    // TODO: optional params knsUpdate? also include tba.
    nodes: std::collections::HashMap<String, net::KnsUpdate>,
    // last block we have an update from
    last_block: u64,
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    let kns_state = fetch_kns_state();

    println!("nodes known: {}", kns_state.nodes.len());

    // now try to message them all
    for name in kns_state.nodes.keys() {
        println!("messaging {name}");
        Request::to((name, "net", "distro", "sys"))
            .body(b"hello")
            .send()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn fetch_kns_state() -> KnsState {
    let Ok(Message::Response { body, .. }) =
        Request::to(("our", "kns_indexer", "kns_indexer", "sys"))
            .body(
                serde_json::json!({
                    "GetState": {
                        "block": 0
                    }
                })
                .to_string()
                .as_bytes()
                .to_vec(),
            )
            .send_and_await_response(60)
            .unwrap()
    else {
        panic!("failed to get response from kns_indexer (GetState)");
    };
    serde_json::from_slice::<KnsState>(&body).unwrap()
}
