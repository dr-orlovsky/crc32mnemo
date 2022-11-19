use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};

use bech32::FromBase32;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file to compute CRC sum.
    ///
    /// Reads STDIN if file if no arguments are given.
    #[clap()]
    pub file: Option<PathBuf>,

    /// Use a valid Bech32 string as an input data
    #[clap(long, conflicts_with = "file")]
    pub bech32: Option<String>,
}

pub fn open_file_or_stdin(filename: Option<impl AsRef<Path>>) -> Result<Box<dyn Read>, io::Error> {
    match filename {
        Some(filename) => {
            let file = fs::File::open(filename)?;
            Ok(Box::new(file))
        }
        None => Ok(Box::new(io::stdin())),
    }
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let data = if let Some(b32) = args.bech32 {
        let (_hrp, encoded, _variant) = bech32::decode(&b32).expect("invalid bech32 data");
        Vec::<u8>::from_base32(&encoded).expect("invalid bech32 data")
    } else {
        let mut file = open_file_or_stdin(args.file)?;
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        data
    };
    // TODO: Use streaming hasher
    let crc32 = crc32fast::hash(&data);
    println!("CRC32 sum: {:08x}", crc32);

    let mut mnemonic = vec![];
    mnemonic::encode(crc32.to_be_bytes(), &mut mnemonic)?;
    let mnemonic = String::from_utf8(mnemonic).expect("mnemonic library error");
    println!("Mnemonic: {}", mnemonic);

    Ok(())
}
