# FORTELION

[![fortelion at crates.io](https://img.shields.io/crates/v/fortelion.svg)](https://crates.io/crates/fortelion)
[![fortelion at docs.rs](https://docs.rs/fortelion/badge.svg)](https://docs.rs/fortelion)

FORTELION is a battery module manufactured by Murata Manufacturing Co., Ltd.
cf. [Product website](https://www.murata.com/ja-jp/products/batteries/stbm)

## Requirement

* libudev-dev

## Communication Interface

All-in-one type of FORTELION battery module has CAN BUS and UART interface.
Currently, only UART library is implemented.
