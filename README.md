# bmkgw

- bmkg wrapper
- for REST API see [bmkgw-api](https://gitlab.com/akane10/bmkgw-api)

## Installation

Cargo.toml

```
[dependencies]
bmkgw = { git = "https://gitlab.com/akane10/bmkgw" }
```

## Example Usage

```rust
use bmkgw::gempa::{self, Gempa, Url};
use bmkgw::cuaca::{self, Province, Domain};
use bmkgw::Error;

async fn main() -> Result<(), Error> {
      let data: Vec<Gempa> = gempa::get_data(Url::GempaTerkini).await?;
      let data1: Option<Url> = Url::from_str("gempaterkini");
      println!("data {:#?}", data);
      println!("data1 {:#?}", data1);

      let data2: Data = cuaca::get_data(Province::DKI).await?;
      let data3: Vec<Domain> = Domain::get_data();
      let data4: Option<Province> = Province::from_str("dki");
      println!("data2 {:#?}", data2);
      println!("data3 {:#?}", data3);
      println!("data4 {:#?}", data4);

      Ok(())
}
```
