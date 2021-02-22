# EMV Tool

EMV provides an easy-to-use interface to interact with EMV smartcards based on this great [PCSC rust library][PCSC lib].

The acronymim EMV stands for Europay-Mastercard-Visa and describes the behaviour of applications running in smart cards.
Most of the specification can be found on Books 1~4 on `./docs/`

Installing
----------

### Homebrew

_Currently only supports macOS_

```shell
brew tap caiotavares/emv
brew install emv
```

### Building from source

Requires [rustup](https://www.rust-lang.org/tools/install)

```shell
git clone git@github.com:caiotavares/emv.git
cargo install --path ./emv
```

Usage
-----

```shell
emv <subcommand>
```

### Subcommands

emv currently the following subcommands

| subcommand | arguments        |
| ---------- | ---------------- |
| `shell`    |                  |
| `run`      | `<input>`        |

Use `shell` if you want `emv` to open a connection with a card and keep listening for APDU commands on the command-line.

If you want `emv` to run a series of APDU commands with minimal manual input, use `run` and provide a path for a file
with APDU command-syntax.

Supported APDU Commands
-----------------------

Some commands require one or two arguments to be provided and for special cases such as `PUT_DATA` and `PIN_UNBLOCK`,
emv will prompt the user to input the cryptographic MAC (message authentication code), since these commands modify card
information.

| APDU                      | Arguments            | Format                         | Prompts for   |
| ---------                 | ---------            | -------------                  | ------------- |
| `SELECT`                  | `<aid>`              | `<hex string>`                 |               | 
| `GET_PROCESSING_OPTIONS`  |                      |                                |               |
| `READ_RECORD`             | `<record>` `<sfi>`   | `<hex string>` `<hex string>`  |               |
| `GET_DATA`                | `<tag>`              | `<hex string>`                 |               |
| `PUT_DATA`                | `<tag>` `<value>`    | `<hex string>`                 |     MAC       |  
| `GENERATE_AC`             | `<type>` `<cdol>`    | `ARQC/TC/ACC`  `<hex string>`  |               |
| `PIN_UNBLOCK`             |                      |                                |     MAC       |

*Example 1:*
Selecting a Mastercard Credit application, fetching the processing options and requesting an ARQC based on a CDOL1:

```shell
$ emv shell
> SELECT A0000000041010
> GET_PROCESSING_OPTIONS
> GENERATE_AC ARQC 0000000000100000000000100986000000000009861504280030901B6A2300001EABC126F85499760000000000000000000000000000000000000000000000000000
```

*Example 2:*
Selecting a Mastercard Maestro application, fetching the processing options, requesting an ARQC and finally requesting a
TC from a CDOL2:

```shell
$ emv shell
> SELECT A0000000043060
> GET_PROCESSING_OPTIONS
> GENERATE_AC ARQC 0000000000100000000000100986000000000009861504280030901B6A2300001EABC126F85499760000000000000000000000000000000000000000000000000000 
> GENERATE_AC TC E014E254B75DBB8D031A3030000000000030901B6A1EABC126F8549976
```

*Example 3:*
Using a PUT DATA command to update a linked application:

```shell
$ emv shell
> SELECT A0000000043060
> GET_PROCESSING_OPTIONS
> GENERATE_AC ARQC 0000000000100000000000100986000000000009861504280030901B6A2300001EABC126F85499760000000000000000000000000000000000000000000000000000 
> GENERATE_AC TC E014E254B75DBB8D031A3030000000000030901B6A1EABC126F8549976
> PUT_DATA DF07 00FF07A000000004101000A5500A4D6173746572636172648701035F2D067074656E65739F1101019F120D4372656469746F204A6169726FBF0C159F5D030100009F4D020B0A9F6E0700760000303000
> Input the MAC: 31E9601F158651AD
```

[PCSC lib]: https://github.com/bluetech/pcsc-rust
