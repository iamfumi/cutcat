use clap::Parser;

///csvファイルの列を操作するためのcatコマンド機能拡張 
#[derive(Parser)]
#[clap(
    name = "cutcat",
    author = "Fumiya YAMAGUCHI",
    version = "0.1.0",
    about = "csvファイルの列を操作するためのcatコマンド機能拡張"
)]
struct Options {

    /// Select Column Name
    #[clap(short='c', long="column", value_name = "column-Name")]
    column: Vec<String>,

    /// Select Column Number
    #[clap(short='n', long="number", value_name = "column-Number")]
    number: Vec<u8>,

    /// Tab delimited csv file
    #[clap(short='t', long="tab")]
    tab: bool,

    /// Input CSV file
    #[clap(value_name = "File", required = true, help = "対象となるCSVファイルのパス")]
    file: Vec<String>,
}

fn main() {
    let _opts = Options::parse();
}