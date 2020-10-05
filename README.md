# EMV Tool

EMV stands for Europay-Mastercard-Visa and describes the behaviour of applications running in smart cards. Most of the specification can be found on Books 1~4 on `./docs/`

This tool provides an easy-to-use interface to interact with EMV smartcards based on this great [PCSC rust library](https://github.com/bluetech/pcsc-rust)

## Requirements

- [rustup](https://www.rust-lang.org/tools/install)

## Getting Started

```bash
git clone git@github.com:caiotavares/emv.git
cd emv
cargo build --release
./target/release/emv
```
