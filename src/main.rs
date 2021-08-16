pub mod bitmap;
pub mod lib;
pub mod types;
pub mod traits;
pub mod encode;
pub mod decode;
mod cli;
mod tests;

use crate::cli::{Opts, SubCommand};
use crate::encode::Encoder;
use crate::decode::Decoder;
use crate::traits::{Decode, Encode};

use clap::Clap;


fn main() {
    let opts: Opts = Opts::parse();
    let file_contents = std::fs::read_to_string(opts.input_file_path).expect("Couldn't read input file");
    let mut output;

    match opts.option {
        SubCommand::Decode => {
            let mut decoder = Decoder::new(&file_contents).unwrap();
            output = decoder.decode().unwrap();
        },
        SubCommand::Encode => {
            let encoder = Encoder::new(file_contents);
            output = encoder.encode().unwrap();
        }
    }

    std::fs::write(opts.output_file_path, output).expect("Couldn't write to file");
    
}
