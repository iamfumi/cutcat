---
title: "Usage"
date: 2022-07-04T01:07:49+09:00
draft: false
weight: 3
header_menu: true
---

## CLI

```sh
cutcat 0.1.2
Fumiya YAMAGUCHI
cat command extension to manipulate columns in csv files

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

### Sample Output

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

## Docker

```sh
docker run --rm -it -v $PWD:/home/cutcat ghcr.io/iamfumi/cutcat:latest
```

The working directory in the docker container is `/home/cutcat`.
The target project should be on the directory with `-v` flag of docker.

### Available versions

* `0.1.2`, `latest`