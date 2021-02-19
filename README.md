# EMV Tool

EMV provides an easy-to-use interface to interact with EMV smartcards based on this great [PCSC rust library][PCSC lib].

The acronymim EMV stands for Europay-Mastercard-Visa and describes the behaviour of applications running in smart cards.
Most of the specification can be found on Books 1~4 on `./docs/`

## Requirements

- [rustup](https://www.rust-lang.org/tools/install)

## Installing

```bash
git clone git@github.com:caiotavares/emv.git
cd emv
cargo build --release
./target/release/emv
```

## Usage

```bash
emv <subcommand>
```

### Subcommands

- `shell`
- `run --input <file>`

Use `shell` if you want `emv` to open a connection with a card and keep listening for APDU commands on the command-line.

If you want `emv` to run a series of APDU commands without manual input, use `run --input <file>`, this will read
through a file of defined commands and only asks for user input when the command requires it.

Available commands:

```bash
SELECT <aid>
GET_PROCESSING_OPTIONS
GET_DATA <tag>
PUT_DATA <tag> <value>
GENERATE_AC <type> <cdol>
```

Example:

```bash
$ emv shell
> SELECT A0000000041010
> GET_PROCESSING_OPTIONS
> GENERATE_AC ARQC 0000000000100000000000100986000000000009861504280030901B6A2300001EABC126F85499760000000000000000000000000000000000000000000000000000
```

[PCSC lib]: https://github.com/bluetech/pcsc-rust
