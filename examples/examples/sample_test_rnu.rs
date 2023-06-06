/// get the block events from the latest block and return the block events, decoded extrinsics
use libuptest::decode_extrinsic::{decode_extrinsic_hex_string, decodec_to_event_summary};
use libuptest::jsonrpseeclient::{JsonrpseeClient, RpcParams};
use libuptest::types::{event_summary, H256};
use libuptest::ws_mod::{get_block_events, get_raw_metadata};

use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Query single block for blockhash to check if it contains a runtime upgrade");
    let client = JsonrpseeClient::with_default_url().unwrap(); // change me
    let metadatablob = get_raw_metadata(client.clone()).await.unwrap();

    // manually pushed a runtime upgrade and copy the block with the block events
    let blockhash: H256 =
        H256::from_str("0x89a5dde6705d345117f442dfacf02f4a59bf5cea3ab713a5c07fc4cd78be3a31")
            .unwrap(); //get_latest_finalized_head(client.clone()).await.unwrap();

    println!("Got block hash: {blockhash:?}");

    let preblock = get_block_events(blockhash, client).await.unwrap();

    let extrinsics = preblock.block.extrinsics;

    let decodedevent_list: Vec<event_summary> = extrinsics
        .clone()
        .iter()
        .map(|n| decodec_to_event_summary(decode_extrinsic_hex_string(n.as_str(), &metadatablob)))
        .collect();

    // Create a custom event that we will search for
    let runtime_upgrade_event: event_summary = event_summary {
        pallet_name: "Sudo".to_string(),
        pallet_method: "sudo_unchecked_weight".to_string(),
    };

    match decodedevent_list.contains(&runtime_upgrade_event) {
        true => println!("Contains runtime upgrade event"),
        false => println!("nope"),
    };
    println!("Looping throw decoded events:");
    for myevent in decodedevent_list {
        println!("decoded event: {:?}", myevent);
    }

    Ok(())
}