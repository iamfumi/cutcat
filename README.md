# cutcat
csvファイルの列を操作するためのcatコマンドの機能拡張

## Description
catコマンドでは，csvファイルの内容も手軽に確認できる．また，cutコマンドではcsvファイルにおける指定した列を取り出すことができる．
しかしながら，これら既存のコマンドは，出力したい何列目かを把握している必要がああり，ファイルが大きくなるにつれ列数の把握が難しくなり，コマンドラインではcsvファイルの列の扱いが難しくなる．
そこで，cutcatでは，指定した列名の列のみを表示する機能を提供する．列名を直接指定できるようにすることで，SQLでデータベースを扱う際のような操作感を提供する．

## Usage

### CLU help message

```sh
cutcat [OPTIONS] <FILE>
OPTIONS
    -c, --column          列名を指定する
    -n, --column-number   列番号を指定する
    -t, --tab             入力ファイルがタブ区切りのcsvファイルの場合に指定する
    -h, --help            このメッセージを表示する

ARGUMENTS
    FILE                  中身の確認を行うcsvファイル
    
```

### Examples

```sh
$ cutcat file.csv
```
