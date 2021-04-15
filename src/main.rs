use std::str::FromStr;
use anyhow::{Result,anyhow, Error};
use structopt::StructOpt;

mod memory_allocator;

use memory_allocator::*;

#[derive(Debug)]
enum AllocationType {
    FIFO,
    LRU,
    OPT
}

impl FromStr for AllocationType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "fifo" => Ok(AllocationType::FIFO),
            "lru" => Ok(AllocationType::LRU),
            "opt" => Ok(AllocationType::OPT),
            _ => Err(anyhow!("Invalid allocation type."))
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct Cli {
    // The number of memory frames that can be used to allocate memory.
    #[structopt(short, long)]
    frames: i32,

    #[structopt(short="t", long="type", default_value="lru")]
    allocation_type: AllocationType,

    // The memory access string, split on spaces.
    #[structopt()]
    reference_string: Vec<String>,
}

fn main() -> Result<()> {
    let Cli {
        reference_string,
        frames,
        allocation_type
    } = Cli::from_args();

    let accesses = reference_string
        .iter()
        .flat_map(|string| string.split(" ").collect::<Vec<_>>())
        .map(|reference| reference.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let (accesses, table) = {
        match allocation_type {
            AllocationType::LRU => {
                let mut lru = LRU::new(frames as usize, accesses.len() as usize);
                let accesses = lru.run(&accesses);
                (accesses, lru.gen_table())
            }
            AllocationType::FIFO => {
                let mut fifo = FIFO::new(frames as usize, accesses.len() as usize);
                let accesses = fifo.run(&accesses);
                (accesses, fifo.gen_table())
            }
            AllocationType::OPT => {
                let mut opt = OPT::new(frames as usize, accesses.len() as usize);
                let accesses = opt.run(&accesses);
                (accesses, opt.gen_table())
            }
        }
    };

    println!("{}", accesses);
    println!("Hits: {} - Misses: {}", accesses.hits(), accesses.misses());
    table.printstd();

    Ok(())
}
