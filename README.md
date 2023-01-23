# vCard Parser

Parses and validates vCard data according to [RFC 6350](https://datatracker.ietf.org/doc/html/rfc6350) specification.

![rust](https://github.com/kenianbei/vcard_parser/actions/workflows/rust.yml/badge.svg)
[![crates](https://img.shields.io/crates/v/vcard_parser.svg)](https://crates.io/crates/vcard_parser)
[![license](https://shields.io/badge/license-MIT-%23373737)](https://github.com/kenianbei/vcard_parser/blob/main/LICENSE)
[![release](https://img.shields.io/github/v/release/kenianbei/vcard_parser)](https://github.com/kenianbei/vcard_parser/tags)

## Installation

Add the library to the dependencies section of your cargo.toml file.

```toml
[dependencies]
vcard_parser = "0.1.2"
```

## Usage

Rust documentation is [here](https://docs.rs/vcard_parser/latest/vcard_parser). 

### Basic Example

Read a vcf file, update the vCard object, and write back to file.

```rust
use std::fs::{read_to_string, write};
use vcard_parser::parse_vcards;
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::value::Value;
use vcard_parser::vcard::value::value_text::ValueTextData;

fn main () {
    let input = read_to_string("contacts.vcf").unwrap_or(String::from("BEGIN:VCARD\nVERSION:4.0\nFN:\nEND:VCARD\n"));
    let mut vcards = parse_vcards(input.as_str()).expect("Unable to parse string.");

    let vcard = vcards.first_mut().unwrap();
    let mut property = vcard.get_property_by_name("FN").unwrap();

    property.set_value(Value::from(ValueTextData::from("John Doe"))).unwrap();
    vcard.set_property(&property).expect("Unable to update property.");

    let mut data = String::new();
    for vcard in vcards {
        data.push_str(vcard.export().as_str())
    }

    write("contacts.vcf", data).expect("Unable to write file.");
}
```
