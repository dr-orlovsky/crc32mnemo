use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};

use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file to compute CRC sum.
    ///
    /// Reads STDIN if file is not given
    #[clap()]
    pub file: Option<PathBuf>,
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

    let mut file = open_file_or_stdin(args.file)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    // TODO: Use streaming hasher
    let crc32 = crc32fast::hash(&data);
    println!("CRC32 sum: {:08x}", crc32);
    let mut mnemonic = vec![];
    mnemonic::encode(crc32.to_be_bytes(), &mut mnemonic)?;
    let mnemonic = String::from_utf8(mnemonic).expect("mnemonic library error");
    println!("Mnemonic: {}", mnemonic);
    Ok(())
}
