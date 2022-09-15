use ethers::prelude::*;
use ethers::providers::Ws;
use eyre::Result;
use std::time::Duration;

mod node;
use crate::node::Node;

abigen!(
    DidWeMergeYet,
    "./out/IDidWeMergeYet.sol/IDidWeMergeYet.json"
);

const TTD: u128 = 58750000000000000000000;

async fn blocks_left(block: &Block<TxHash>) -> Option<u128> {
    let difficulty = block.difficulty;
    let total_difficulty = block.total_difficulty.unwrap();

    if total_difficulty > U256::from(TTD) {
        Some(0)
    } else {
        let difficulty = difficulty.as_u128();
        let total_difficulty = total_difficulty.as_u128();

        let left = (TTD - total_difficulty) / difficulty;
        Some(left)
    }
}

async fn does_oracle_exist(node: &Node) -> Result<bool> {
    let address: Address = "0xD6a6f0D7f08c2D31455a210546F85DdfF1D9030a".parse()?;
    Ok(node.client.get_code(address, None).await?.len() > 0)
}

#[tokio::main]
async fn main() -> Result<()> {
    let node = Node::new_local_node_from_env().await?;
    let ws = Ws::connect(node.ws_endpoint.clone()).await?;
    let provider = Provider::new(ws).interval(Duration::from_millis(1000));

    println!("Watching for blocks");
    let address: Address = "0xc86E1A7a4AA5A9B17f6997a59B311835fc95e975".parse()?;
    let did_we_merge_yet = DidWeMergeYet::new(address, node.client.clone());

    println!(
        "Does oracle exist yet: {:?}",
        does_oracle_exist(&node).await?
    );

    let mut stream = provider.watch_blocks().await?;
    while let Some(block) = stream.next().await {
        let block = provider.get_block(block).await?.unwrap();
        println!(
            "Ta: {:?}, block number: {} difficulty: {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.difficulty,
        );

        if let Some(num) = blocks_left(&block).await {
            println!("Blocks away: {:?}", num);
            if num <= 3 {
                // TODO test on anvil
                if let Ok(tx) = did_we_merge_yet.trigger().send().await?.await {
                    println!("Sent transaction: {:?}", tx);
                } else {
                    println!("A transaction failed");
                    if does_oracle_exist(&node).await? {
                        println!("Oracle exists. Too late.");
                    }
                }
            } else {
                println!("Did nothing");
            }
        }
    }

    Ok(())
}
