use clap::Parser;
use std::path::PathBuf;
use std::fs;

struct Options {

    /// Select Column Name
    #[clap(short='c', long="column", value_name = "column-Name")]
    column: Vec<String>,

    /// Select Column Number
    #[clap(short='n', long="number", value_name = "column-Number")]
    number: Vec<usize>,

    /// Tab delimited csv file
    #[clap(short='t', long="tab")]
    tab: bool,

    /// Input CSV file
    #[clap(value_name = "File", required = true, help = "対象となるCSVファイルのパス")]
    file: PathBuf,
}
