use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "Ran Z. <rantzvi@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// The input file path
    #[clap(short, long)]
    pub input_file_path: String,

    /// The output file path
    #[clap(short, long)]
    pub output_file_path: String,

    // Encode or Decode
    #[clap(subcommand)]
    pub option: SubCommand

}
#[derive(Clap)]
pub enum SubCommand {
    Decode,
    Encode
}