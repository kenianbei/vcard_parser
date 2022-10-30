# vCard Parser

Parses and validates vCard data according to RFC 6350 specification.

![rust](https://github.com/kenianbei/vcard_parser/actions/workflows/rust.yml/badge.svg)
[![crates](https://img.shields.io/crates/v/vcard_parser.svg)](https://crates.io/crates/vcard_parser)
[![license](https://shields.io/badge/license-MIT-%23373737)](https://github.com/kenianbei/vcard_parser/blob/main/LICENSE)
[![release](https://img.shields.io/github/v/release/kenianbei/vcard_parser)](https://github.com/kenianbei/vcard_parser/tags)

## Installation

Add the library to the dependencies section of your cargo.toml file.

```toml
[dependencies]
vcard_parser = "0.1.0"
```

## Usage

Rust documentation is [here](https://docs.rs/vcard_parser/latest/vcard_parser). There are two main functions for parsing vCards from a file or input string:

* [parse_to_vcards](https://docs.rs/vcard_parser/latest/vcard_parser/fn.parse_to_vcards.html): Parses vCards from a string, failing on any error.
* [parse_to_vcards_without_errors](https://docs.rs/vcard_parser/latest/vcard_parser/fn.parse_to_vcards_without_errors.html): Parses vCards from a string, ignoring properties that throw an error.

Typically, you will use [parse_to_vcards_without_errors](https://docs.rs/vcard_parser/latest/vcard_parser/fn.parse_to_vcards_without_errors.html), unless you are sure that the input doesn't contain x-param or iana-token properties.

### Parsing vCards

Reading a vcf file, updating the vCard object, and writing back to the file.

```rust
use std::fs;
use std::fs::read_to_string;
use vcard_parser::parse_to_vcards_without_errors;
use vcard_parser::vcard::property::types::PropertyType;

fn main () {
    if let Ok(string) = read_to_string("contacts.vcf") {
        let mut vcards = parse_to_vcards_without_errors(string.as_str());

        let mut vcard = vcards.first().unwrap().clone();
        let property = vcard.get_property_by_type(&PropertyType::Fn).unwrap();

        vcard.update_property(property.get_uuid(), "FN:John Doe").expect("Unable to update property.");
        vcards[0] = vcard;

        let mut data = String::new();
        for vcard in vcards {
            data.push_str(vcard.to_string().as_str())
        }
        fs::write("contacts.vcf", data).expect("Unable to write file.");
    }
}
```

### Creating a new vCard

```rust
use vcard_parser::vcard::Vcard;

fn main () {
    let mut vcard = Vcard::default();
    vcard.add_property("NICKNAME:Johnny").unwrap();
    println!("{}", vcard.to_string());
}
```

### Parsing a single vCard without error checking

```rust
use vcard_parser::vcard::Vcard;

fn main () {
    let mut vcard = Vcard::from("VERSION:4.0\nFN:John Doe\n");
    vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    println!("{}", vcard.to_string());
}
```

### Parsing a single vCard with error checking

```rust
use vcard_parser::vcard::Vcard;

fn main () {
    let mut vcard = Vcard::try_from("VERSION:4.0\nFN:John Doe\n").expect("Unable to parse input.");
    vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    println!("{}", vcard.to_string());
}
```
