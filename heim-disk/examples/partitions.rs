use heim_common::prelude::*;
use heim_disk as disk;

#[heim_derive::main]
async fn main() -> Result<()> {
    println!("---- Partitions ----");
    let mut partitions = disk::partitions();
    while let Some(part) = partitions.next().await {
        dbg!(part?);
    }

    println!("---- Physical partitions ----");

    let mut partitions = disk::partitions();
    while let Some(part) = partitions.next().await {
        dbg!(part?);
    }

    Ok(())
}
