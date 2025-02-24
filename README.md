# Deadpool for Ldap [![Latest Version](https://img.shields.io/crates/v/deadpool-ldap3.svg)](https://crates.io/crates/deadpool-ldap3) ![Unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg "Unsafe forbidden") [![Rust 1.75+](https://img.shields.io/badge/rustc-1.75+-lightgray.svg "Rust 1.75+")](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

Deadpool is a dead simple async pool for connections and objects
of any type.

This crate implements a [`deadpool3`](https://crates.io/crates/deadpool)
manager for [`ldap3`](https://crates.io/crates/ldap3)
and provides a wrapper that ensures correct use of the connection
inside a separate thread.

## Features

| Feature | Description | Extra dependencies | Default |
| ------- | ----------- | ------------------ | ------- |
| `rt_tokio_1` | Enable support for [tokio](https://crates.io/crates/tokio) crate | `deadpool/rt_tokio_1` | yes |
| `rt_async-std_1` | Enable support for [async-std](https://crates.io/crates/config) crate | `deadpool/rt_async-std_1` | no |
| `serde` | Enable support for [serde](https://crates.io/crates/serde) crate | `deadpool/serde`, `serde/derive` | no |
| `tracing` | Enable support for [tracing](https://github.com/tokio-rs/tracing) by propagating Spans in the `interact()` calls. Enable this if you use the `tracing` crate and you want to get useful traces from within `interact()` calls. | `deadpool-sync/tracing`, `tracing` | no |

## Example

```rust
use deadpool_ldap::{Config, Runtime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(result, 1);
    let cfg = Config {
        url: "ldap://127.0.0.1:389".to_string(),
        bind_dn: Some("cn=admin,dc=demo,dc=com".to_string()),
        bind_password: Some("123456".to_string()),
        pool: None,
    };
    let pool = cfg.create_pool(Runtime::Tokio1).unwrap();
    let mut conn = pool.get().await.unwrap();
    conn.simple_bind("admin", "123456").await.unwrap();
    assert_eq!(result, 1);
    Ok(())
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
