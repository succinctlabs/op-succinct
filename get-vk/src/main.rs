use clap::Parser;
use sp1_sdk::{HashableKey, ProverClient};
use std::{fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The ELF file to get the VK for.
    #[arg(short, long)]
    elf: String,
}

/// Get the verification key for a given program.
fn main() {
    let args = Args::parse();

    // Read the elf file contents into a Vec<u8>
    let mut file = File::open(format!("elf/{}", args.elf)).unwrap();
    let mut elf_contents = Vec::new();
    file.read_to_end(&mut elf_contents).unwrap();

    let prover = ProverClient::new();
    let (_, vk) = prover.setup(&elf_contents);
    println!("{:?}", vk.vk.bytes32());
}
