# hamdb-rs

Minimal async client for the public [HamDB](https://hamdb.org) amateur-radio callsign lookup service.

## Installation
Add the crate to your project using Cargo:

```
cargo add hamdb
```

## Usage
```rust
use hamdb::v1::Client;

#[tokio::main]
async fn main() -> Result<(), hamdb::Error> {
    let client = Client::new("app-name-here");
    let info = client.lookup("W4AQL").await?;

    println!("{} {}", info.call, info.country);
    Ok(())
}
```

If you only need to validate user input before sending it to another system, reuse the parser directly:

```rust
use hamdb::parsing::parse_callsign;

let parsed = parse_callsign("W1AW/AE")?;
assert_eq!(parsed.base, "W1AW");
assert_eq!(parsed.suffix.as_deref(), Some("AE"));
```

## Contributing
Contributions are welcome. Feel free to open an issue or PR for any new features or concerns that you have.

## License
Licensed under the MIT License; see `LICENSE` for the full text. This crate has no affiliation with the HamDB project or its maintainers, and HamDB retains all rights to its API, data, and trademarks. Use of this library is subject to HamDB's published policies and any integrations you build must respect their terms of use.
