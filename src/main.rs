use clap::Parser;
use std::path::PathBuf;
use std::fs;

///csvファイルの列を操作するためのcatコマンド機能拡張 
#[derive(Parser)]
#[clap(
    name = "cutcat",
    author = "Fumiya YAMAGUCHI",
    version = "0.1.2",
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

fn readfile(path_buf: PathBuf) -> String {
    return fs::read_to_string(path_buf).unwrap();
}

fn textsplit(text: &str) -> Vec<&str> {
    let line :Vec<&str> = text.split('\n').collect();
    let mut one: Vec<&str> = vec![];
    for x in line{
        // println!("{}", x);
        let split_line :Vec<&str> = x.split(',').collect();
        for splited in split_line{
            one.push(splited);
        }
    }
    return one;
}

// fn rowsplit(csvdata: &str, num: u8) -> Vec<&str> {

// }

fn main() {
    let _opts = Options::parse();
    //let path_buf = PathBuf::from("testdata.csv");
    let path_buf = _opts.file;
    let csvtext = readfile(path_buf);
    println!("{}",csvtext);
    let str2: &str = &csvtext;
    // println!("{}", str2);
    let test_string :Vec<&str> = textsplit(str2);
    // println!("{},{}",test_string[0],test_string[1]);

    for z in test_string{
        println!("{}", z);
    }

    // let row :Vec<&str> = rowsplit(str2, 1);
    // println!("{},{}",row[0],row[1]);

    // let column :Vec<String> = _opts.column;
    // println!("{}",column[0]);
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