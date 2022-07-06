# cutcat

[![Coverage Status](https://coveralls.io/repos/github/iamfumi/cutcat/badge.svg?branch=main)](https://coveralls.io/github/iamfumi/cutcat?branch=main)
[![build](https://github.com/iamfumi/cutcat/actions/workflows/build.yaml/badge.svg)](https://github.com/iamfumi/cutcat/actions/workflows/build.yaml)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/iamfumi/cutcat)](https://rust-reportcard.xuri.me/report/github.com/iamfumi/cutcat)

[![Version](https://img.shields.io/badge/Version-v0.1.0-green)](https://github.com/iamfumi/cutcat/releases/tag/v0.1.0)
[![Docker](https://img.shields.io/badge/Docker-v0.1.0-green?logo=docker)](https://github.com/iamfumi/cutcat/pkgs/container/cutcat/)
[![Homebrew](https://img.shields.io/badge/Homebrew-iamfumi/brew/cutcat-green?logo=homebrew)](https://github.com/iamfumi/homebrew-brew)

[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/iamfumi/cutcat/blob/main/LICENSE)
[![DOI](https://sandbox.zenodo.org/badge/483074563.svg)](https://sandbox.zenodo.org/badge/latestdoi/483074563)

csvファイルの列を操作するためのcatコマンド機能拡張
![cutcat_logo](logo.png)

## Description
catコマンドでは，csvファイルの内容も手軽に確認できる．また，cutコマンドではcsvファイルにおける指定した列を取り出すことができる．
しかしながら，これら既存のコマンドでは，出力したい列が何列目かを把握している必要があり，ファイルが大きくなるにつれ把握が難しくなることから，コマンドラインではcsvファイルの列の扱いが難しくなる．
そこで，cutcatでは，指定した列名の列のみを表示する機能を提供する．列名を直接指定できるようにすることで，SQLでデータベースを扱う際のような操作感を提供する．

## Usage

```sh
USAGE:
    cutcat [OPTIONS] <File>

ARGS:
    <File>    対象となるCSVファイルのパス

OPTIONS:
    -c, --column <column-Name>      Select Column Name
    -h, --help                      Print help information
    -n, --number <column-Number>    Select Column Number
    -t, --tab                       Tab delimited csv file
    -V, --version                   Print version information
    
```

### Examples

```sh
$ cutcat SalesData.csv
Products Name, 2017, 2018, 2019, 2020, 2021
A, 35000, 38000, 46000, 12000, 36000
B, 9000, 20000, 23100, 54300, 12000
C, 42300, 54300, 43200, 89100, 123200

$ cutcat -c "2021" SalesData.csv
2021
36000
12000
123200

$ cutcat -c "Products Name" -c "2021" SalesData.csv
Products Name, 2021
A, 36000
B, 12000
C, 123200

$ cutcat -n 0 -n 4 -n 5 SalesData.csv
Products Name, 2020, 2021
A, 12000, 36000
B, 54300, 12000
C, 89100, 123200

```
