[![Clippy](https://github.com/nogibjj/Gavin_Li_Week7_Mini_Project/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/Gavin_Li_Week7_Mini_Project/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/Gavin_Li_Week7_Mini_Project/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/Gavin_Li_Week7_Mini_Project/actions/workflows/tests.yml)

# IDS 706 Data Engineering Week 7 Mini Project

Gavin Li `gl183`

## Purpose of the project

The purpose of this project is to build a rust command line tool as part of the rust lab.

## Result for `make format`, `make lint`, `make test`

- Little bit different from make commands for python projects, for this project, go to the `count-num-cli` folder using `cd` command, then call make commands to work solely on this one project.

![make_rslt](./resources/make_rslts.png)

## Guideline for my command line tool

The general purpose of my cli tool is to count the occurance of elements in a list (which is helpful for NLP class🤨).

There are two flags provided to use the tool.

1. `--default`

- counts the occurance of integers in a default list.

2. `--cust-list`

- counts the occurance of integers in a user customized list.
- if there is a `--default` flag, `--cust-list` will be ignored.
- if there is only a `cust-list` flag, an input list will be required.
- if the user input contains characters other than integer and comma, only the valid part will be parsed to integers and counted.

## References

* [Professor Noah's rust data engineering repo](https://github.com/nogibjj/rust-data-engineering)
