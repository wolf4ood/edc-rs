
<div class="oranda-hide">
  <h1 align="center">EDC-rs</h1>
</div>

<div align="center">
  <strong>
    Rust client and tools for <a href="https://github.com/eclipse-edc/Connector">EDC</a>.
  </strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/dataspace-rs/edc-rs?query=workflow%3ATests">
    <img src="https://github.com/dataspace-rs/edc-rs/workflows/Tests/badge.svg"
    alt="Tests status" />
  </a>
  
  <a href="https://crates.io/crates/edc-connector-client">
    <img src="https://img.shields.io/crates/d/edc-connector-client.svg?style=flat-square"
      alt="Download" />
  </a>
  <a href="https://docs.rs/edc-connector-client">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>

   <a href="https://opensource.org/licenses/Apache-2.0">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg"
      alt="license" />
  </a>

   <a href="https://deps.rs/repo/github/dataspace-rs/edc-rs">
    <img src="https://deps.rs/repo/github/dataspace-rs/edc-rs/status.svg"
      alt="license" />
  </a>

</div>


## edc-connector-client 

A Rust client for [EDC](https://github.com/eclipse-edc/Connector).



### Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
edc-connector-client = "0.1"
```


### Examples


#### Basic usage


Fetching an asset with id `1` and reading the `description` property as string.

```rust
use edc_connector_client::{Auth, EdcConnectorClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = EdcConnectorClient::builder()
        .management_url("http://myedc")
        .with_auth(Auth::api_token("password"))
        .build()?;

    let asset = client.assets().get("1").await?;

    println!("Got {:?}", asset);

    println!(
        "Property description: {:?}",
        asset.property::<String>("description").unwrap()
    );

    Ok(())
}
```


### Development


#### Compiling

```
git clone https://github.com/dataspace-rs/edc-rs.git
cd edc-rs
cargo build
```


#### Testing 

Some tests run against a running instance of EDC.

You can use docker compose to start an instance for testing. 

```
docker compose -f testing/docker-compose.yml up -d
cargo test 
```

The tests setup was mostly derived by the Typescript client [edc-connector-client](https://github.com/Think-iT-Labs/edc-connector-client)

