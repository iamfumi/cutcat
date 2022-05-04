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
    /// Name of the person to greet
    // #[clap(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[clap(short, long, default_value_t = 1)]
    // count: u8,

    /// Select Column Name
    #[clap(short, long, value_name = "column-Name")]
    column: Option<String>,

    /// Select Column Number
    #[clap(short, long, value_name = "column-Number")]
    number: Option<u8>,   

    /// Input CSV file
    #[clap(value_name = "File", required = true, help = "対象となるCSVファイルのパス")]
    file: Vec<String>,
}

fn main() {
    let opts = Options::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }
}