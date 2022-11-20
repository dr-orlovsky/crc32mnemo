#[macro_use]
extern crate amplify;

use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::string::FromUtf8Error;

use bech32::FromBase32;
use clap::Parser;
use colorize::AnsiColor;

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

#[derive(Debug, Display, Error, From)]
#[display(inner)]
pub enum Error {
    #[from]
    Io(io::Error),

    #[display("incorrect bech32(m) string due to {0}")]
    #[from]
    Bech32(bech32::Error),

    #[from]
    Utf8(FromUtf8Error),
}

fn main() {
    let args = Args::parse();

    if let Err(err) = args.exec() {
        eprintln!("{}: {}", "Error".red(), err);
    }
}

impl Args {
    fn exec(self) -> Result<(), Error> {
        let data = if let Some(b32) = self.bech32 {
            let (_hrp, encoded, _variant) = bech32::decode(&b32)?;
            Vec::<u8>::from_base32(&encoded)?
        } else {
            let mut file = open_file_or_stdin(self.file)?;
            let mut data = vec![];
            file.read_to_end(&mut data)?;
            data
        };
        // TODO: Use streaming hasher
        let crc32 = crc32fast::hash(&data);
        println!("CRC32 sum: {:08x}", crc32);

        let mut mnemonic = vec![];
        mnemonic::encode(crc32.to_be_bytes(), &mut mnemonic)?;
        let mnemonic = String::from_utf8(mnemonic)?;
        println!("Mnemonic: {}", mnemonic);
        Ok(())
    }
}
