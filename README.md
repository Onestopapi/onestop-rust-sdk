# OneStop Rust SDK

Rust client for the [OneStop Antibot](https://onestopantibot.com) API. The SDK calls `/generate` and returns typed results; your application fetches the target pages and scripts and sends the generated payloads.

## Install

```sh
cargo add onestop-antibot
cargo add tokio --features macros,rt-multi-thread
```

The crate is `onestop-antibot`; its library name is `onestop`. Requires Rust 1.88+, Tokio and a C toolchain (zstd builds a bundled C library). Package: [crates.io](https://crates.io/crates/onestop-antibot), API reference: [docs.rs](https://docs.rs/onestop-antibot).

## Quick start

```rust
let client = onestop::Client::new(onestop::Config {
    api_key: "YOUR_API_KEY".into(),
    identifier: "".into(), // your identifier or account name
    ..Default::default()
})?;
let ip = client.get_ip().await?; // set Config::ip_http_client to your proxy so this is the proxy's IP
let session = client.session(onestop::Session {
    site: page_url.clone(), referer: page_url, user_agent, accept_language: "en-US,en;q=0.9".into(), ip,
});
let res = session.akamai_sensor(&onestop::AkamaiSensorCall { abck, bmsz, script, script_url, ..Default::default() }).await?;
// POST res.payload as sensor_data; send res.context on the next call.
```

Documentation: [onestopantibot.com/dashboard/docs](https://onestopantibot.com/dashboard/docs)
