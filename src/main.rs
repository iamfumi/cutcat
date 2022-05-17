use clap::Parser;
use std::path::PathBuf;
use std::fs;

///csvファイルの列を操作するためのcatコマンド機能拡張 
#[derive(Parser)]
#[clap(
    name = "cutcat",
    author = "Fumiya YAMAGUCHI",
    version = "0.1.1",
    about = "csvファイルの列を操作するためのcatコマンド機能拡張"
)]
struct Options {

    /// Select Column Name
    #[clap(short='c', long="column", value_name = "column-Name...")]
    column: Vec<String>,

    /// Select Column Number
    #[clap(short='n', long="number", value_name = "column-Number...")]
    number: Vec<u8>,

    /// Tab delimited csv file
    #[clap(short='t', long="tab")]
    tab: bool,

    /// Input CSV file
    #[clap(value_name = "File", required = true, help = "対象となるCSVファイルのパス")]
    file: PathBuf,
}

fn main() {
    let _opts = Options::parse();
    //let path_buf = PathBuf::from("testdata.csv");
    let path_buf = _opts.file;
    let csvtext = readfile(path_buf);
    println!("{}",csvtext);
}

fn readfile(path_buf: PathBuf) -> String {
    return fs::read_to_string(path_buf).unwrap();
}

fn hello(name: Option<String>) -> String {
    return format!("Hello, {}", if let Some(n) = name {
        n
    } else {
        "World".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("Hello, World", hello(None));
        assert_eq!("Hello, Yamaguchi", hello(Some("Yamaguchi".to_string())));
    }
}