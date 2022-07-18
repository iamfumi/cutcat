---
title: "Install"
date: 2022-07-03T18:20:54+09:00
weight: 2
draft: false
header_menu: true
---

### Homebrew

`cutcat` supports installation via [Homebrew](https://brew.sh/).

```sh
$ brew tap iamfumi/fm
$ brew install cutcat
```

### Install yourself

Clone the project from GitHub and compile it like follows.

```sh
$ git clone https://github.com/iamfumi/cutcat.git
$ cd cutcat
$ cargo build --release
```

Then, `cargo` build the resultant `cutcat` executable on `target/release` directory.
