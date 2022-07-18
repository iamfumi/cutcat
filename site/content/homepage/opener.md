---
title: "cutcat"
weight: 1
header_menu: true
---

[![Coverage Status](https://coveralls.io/repos/github/iamfumi/cutcat/badge.svg?branch=main)](https://coveralls.io/github/iamfumi/cutcat?branch=main)
[![build](https://github.com/iamfumi/cutcat/actions/workflows/build.yaml/badge.svg)](https://github.com/iamfumi/cutcat/actions/workflows/build.yaml)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/iamfumi/cutcat)](https://rust-reportcard.xuri.me/report/github.com/iamfumi/cutcat)

[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/iamfumi/cutcat/blob/main/LICENSE)
[![DOI](https://sandbox.zenodo.org/badge/483074563.svg)](https://sandbox.zenodo.org/badge/latestdoi/483074563)

csvファイルの列を操作するためのcatコマンド機能拡張
![cutcat_logo](./logo.png)

## Description
catコマンドを用いることで，csvファイルの内容も手軽に確認できます．
また，cutコマンドを用いるとcsvファイルにおける指定した列を取り出すことができます．

しかしながら，これら既存のコマンドでは，出力したい列が何列目かを把握している必要があります．
つまり，ファイルが大きくなると共にデータの把握が難しくなることから，コマンドラインではcsvファイルの列の扱いが難しくなります．

そこで，cutcatでは，指定した列名の列のみを表示する機能を提供します．列名を直接指定できるようにすることで，SQLでデータベースを扱う際のような操作感を提供します．

