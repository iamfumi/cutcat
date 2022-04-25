# cutcat
csvファイルの列を操作するためのcatコマンド機能拡張
![cutcat_logo](logo.png)

## Description
catコマンドでは，csvファイルの内容も手軽に確認できる．また，cutコマンドではcsvファイルにおける指定した列を取り出すことができる．
しかしながら，これら既存のコマンドでは，出力したい列が何列目かを把握している必要があり，ファイルが大きくなるにつれ把握が難しくなることから，コマンドラインではcsvファイルの列の扱いが難しくなる．
そこで，cutcatでは，指定した列名の列のみを表示する機能を提供する．列名を直接指定できるようにすることで，SQLでデータベースを扱う際のような操作感を提供する．

## Usage

```sh
cutcat [OPTIONS]... <FILE>...
OPTIONS
    -c, --column          列名を指定する
    -n, --column-number   列番号を指定する
    -t, --tab             入力ファイルがタブ区切りのcsvファイルの場合に指定する
    -h, --help            このメッセージを表示する

FILE
    FILE                  対象となるcsvファイルのパス
    
```

### Examples

```sh
$ cutcat file.csv
```
