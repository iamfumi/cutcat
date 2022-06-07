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

fn returnline(text: &str) -> Vec<&str> {
    let line :Vec<&str> = text.split('\n').collect();
    return line;
}

// fn getcolmun(text: &str) -> Vec<&str> {

// }

fn rowsplit(line:&str) -> Vec<&str> {
    let word :Vec<&str> = line.split(',').collect();
    return word;
}

fn cleateresult(mut result:String, lines: Vec<&str>, number:Vec<usize>) -> String{
    let ll = lines.len();
    let mut cl = 0;
    for line in lines{
        let ones :Vec<&str>= rowsplit(line);
        let wl = number.len();
        let mut cw = 0;
        for n in &number{
            cw+=1;
            result = result + ones[*n];
            if wl != cw{
                result = result+",";
            }
        }
        cl+=1;
        if cl!=ll {
            result = result + "\n";
        }
        // println!("{}",line);
    }
    return result;
}

fn collectnum(mut number:Vec<usize>, column:Vec<String>, head:&str) -> Vec<usize>{
    let heads: Vec<&str> = rowsplit(head);
    for onec in column{
        let mut counter = 0;
        for oneh in &heads{
            if &onec == oneh{
                number.push(counter);
            }
            counter+=1;
        }
    }
    return number;
}

fn main() {
    let _opts = Options::parse();
    //let path_buf = PathBuf::from("testdata.csv");
    let path_buf = _opts.file;
    let csvtext = readfile(path_buf);
    // println!("{}",csvtext);
    let str2: &str = &csvtext;
    // println!("{}", str2);
    let allsplited :Vec<&str> = textsplit(str2);
    // println!("{},{}",test_string[0],test_string[1]);

    // for z in allsplited{
    //     println!("{}", z);
    // }

    // let row :Vec<&str> = rowsplit(str2, 1);
    // println!("{},{}",row[0],row[1]);

    let column :Vec<String> = _opts.column;
    // if column.len()==0{
    //     println!("{}","None");
    // }else{
    //     for x in column{
    //         println!("{}",x);
    //     }
    // }
    let lines :Vec<&str> = returnline(str2);
    let mut number :Vec<usize> = _opts.number;
    
    
    let mut result:String = String::from("");
    if column.len() !=0 {
        number = collectnum(number, column, lines[0]);
    }
    if number.len() !=0 {
        result = cleateresult(result, lines, number);
    }else{
        result = csvtext;
    }

    println!("{}",result);

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
    #[test]
    fn test_rowsplit(){
        let testnum: &str = "1,2";
        let teststr: &str = "c,d";
        let a: &str = "1";
        let b: &str = "2";
        let c: &str = "c";
        let d: &str = "d";
        assert_eq!(vec![a,b], rowsplit(testnum));
        assert_eq!(vec![c,d], rowsplit(teststr));
    }
    #[test]
    fn test_returnline(){
        let testreturn: &str = "1\n2";
        let a: &str = "1";
        let b: &str = "2";
        let teststr: &str = "c\nd\ne";
        let c: &str = "c";
        let d: &str = "d";
        let e: &str = "e";
        assert_eq!(vec![a,b], returnline(testreturn));
        assert_eq!(vec![c,d,e], returnline(teststr));
    }
    #[test]
    fn test_collectnum(){
        let mut number:Vec<usize> = vec![0];
        let column:Vec<String> = vec!["B".to_string(),"C".to_string()];
        let head:&str = "A,B,C";
        assert_eq!(vec![0,1,2],collectnum(number, column, head))
    }
    #[test]
    fn test_readfile(){
        let path_buf = PathBuf::from("./SalesData.csv");
        assert_eq!("Products Name,2017,2018,2019,2020,2021\r\nA,35000,38000,46000,12000,36000\r\nB,9000,20000,23100,54300,12000\r\nC,42300,54300,43200,89100,123200",readfile(path_buf));
    }
}