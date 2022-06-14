use clap::Parser;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[clap(author, version, about)]

pub struct Options {

    /// Select Column Name
    #[clap(short='c', long="column", value_name = "column-Name")]
    pub column: Vec<String>,

    /// Select Column Number
    #[clap(short='n', long="number", value_name = "column-Number")]
    pub number: Vec<usize>,

    /// Tab delimited csv file
    #[clap(short='t', long="tab")]
    pub tab: bool,

    /// Input CSV file
    #[clap(value_name = "File", required = true, help = "対象となるCSVファイルのパス")]
    pub file: PathBuf,
}
