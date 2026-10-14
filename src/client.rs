use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_TYPE, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::types::{Meta, Session};
use crate::wire::encode;
use crate::Error;

/// Default API endpoint.
pub const DEFAULT_ENDPOINT: &str = "https://onestopantibot.com/generate";
const IP_ENDPOINT: &str = "https://ip.onestopantibot.com/";

/// API connection settings. Browser identity belongs in [`Session`].
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Sent as `x-key`. Required.
    pub api_key: String,
    /// Sent as `User-Agent` on API requests. Required.
    pub identifier: String,
    /// Optional, sent as `x-sub-key`.
    pub sub_key: String,
    /// Used as given; empty uses [`DEFAULT_ENDPOINT`].
    pub endpoint: String,
    /// `zstd` (default when empty), `gzip` or `identity`.
    pub compression: String,
    /// Whole-request deadline for every call. Zero means 60 seconds.
    pub timeout: Duration,
    /// API transport. The default ignores proxy environment variables.
    pub http_client: Option<reqwest::Client>,
    /// Used by [`Client::get_ip`], usually your target proxy. Defaults to `http_client`.
    pub ip_http_client: Option<reqwest::Client>,
    /// Merged into every session; non-empty session fields win.
    pub defaults: Session,
    /// Sends every API call through this proxy URL, such as `http://127.0.0.1:8888`, on the SDK's own client
    /// (`http_client` is not used) and accepts its certificate. `get_ip` is unaffected.
    pub debug_proxy: String,
}

#[derive(Clone, Copy)]
enum Compression {
    Zstd,
    Gzip,
    Identity,
}

/// OneStop API client. Cheap to clone; reuse one across concurrent calls.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    api_key: String,
    identifier: String,
    sub_key: String,
    endpoint: String,
    compression: Compression,
    timeout: Duration,
    http: reqwest::Client,
    ip_http: reqwest::Client,
    defaults: Session,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").field("endpoint", &self.inner.endpoint).field("identifier", &self.inner.identifier).finish_non_exhaustive()
    }
}

impl Client {
    /// Requires an API key and identifier.
    pub fn new(config: Config) -> Result<Self, Error> {
        if config.api_key.trim().is_empty() || config.identifier.trim().is_empty() {
            return Err(Error::local("onestop: APIKey and Identifier are required"));
        }
        let compression = match config.compression.as_str() {
            "" | "zstd" => Compression::Zstd,
            "gzip" => Compression::Gzip,
            "identity" => Compression::Identity,
            _ => return Err(Error::local("onestop: compression must be zstd, gzip or identity")),
        };
        let endpoint = if config.endpoint.is_empty() { DEFAULT_ENDPOINT.to_string() } else { config.endpoint };
        let timeout = if config.timeout.is_zero() { Duration::from_secs(60) } else { config.timeout };
        let mut http = config.http_client.unwrap_or_else(|| reqwest::Client::builder().no_proxy().build().unwrap());
        let ip_http = config.ip_http_client.unwrap_or_else(|| http.clone());
        if !config.debug_proxy.is_empty() {
            let proxy = reqwest::Proxy::all(&config.debug_proxy).map_err(|e| failure("request", &e))?;
            http = reqwest::Client::builder().proxy(proxy).danger_accept_invalid_certs(true).build().unwrap();
        }
        Ok(Self {
            inner: Arc::new(Inner {
                api_key: config.api_key,
                identifier: config.identifier,
                sub_key: config.sub_key,
                endpoint,
                compression,
                timeout,
                http,
                ip_http,
                defaults: config.defaults,
            }),
        })
    }

    /// Returns the public IP seen by `https://ip.onestopantibot.com/`, using [`Config::ip_http_client`].
    pub async fn get_ip(&self) -> Result<String, Error> {
        let request = self.inner.ip_http.get(IP_ENDPOINT).timeout(self.inner.timeout);
        let response = self.auth(request).send().await.map_err(|e| failure("request", &e))?;
        if response.status() != reqwest::StatusCode::OK {
            return Err(Error::api(response.status().as_u16(), "", "IP lookup failed"));
        }
        let body = response.text().await.map_err(|e| failure("read response", &e))?;
        Ok(body.trim().to_string())
    }

    /// Returns [`Config::defaults`] merged with `session`; non-empty `session` fields win.
    pub fn session(&self, session: Session) -> ClientSession {
        let base = &self.inner.defaults;
        let pick = |value: String, fallback: &String| if value.is_empty() { fallback.clone() } else { value };
        ClientSession {
            client: self.clone(),
            session: Session {
                site: pick(session.site, &base.site),
                referer: pick(session.referer, &base.referer),
                user_agent: pick(session.user_agent, &base.user_agent),
                accept_language: pick(session.accept_language, &base.accept_language),
                ip: pick(session.ip, &base.ip),
            },
        }
    }

    fn auth(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request = request.header("x-key", &self.inner.api_key);
        if !self.inner.sub_key.is_empty() {
            request = request.header("x-sub-key", &self.inner.sub_key);
        }
        request.header(USER_AGENT, &self.inner.identifier)
    }

    pub(crate) async fn generate<I: Serialize, T: Meta + DeserializeOwned + Default>(
        &self,
        mode: &str,
        provider: &str,
        input: &I,
    ) -> Result<T, Error> {
        let mut body = encode(&BTreeMap::from([(format!("{provider}Request"), input)])).map_err(|e| failure("encode request", &e))?;
        let mut request =
            self.inner.http.post(&self.inner.endpoint).timeout(self.inner.timeout).header(CONTENT_TYPE, "application/json; charset=utf-8");
        match self.inner.compression {
            Compression::Zstd => {
                let mut compressor = zstd::bulk::Compressor::new(7).unwrap();
                compressor.include_checksum(true).unwrap();
                body = compressor.compress(&body).unwrap();
                request = request.header(CONTENT_ENCODING, "zstd");
            }
            Compression::Gzip => {
                let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
                encoder.write_all(&body).unwrap();
                body = encoder.finish().unwrap();
                request = request.header(CONTENT_ENCODING, "gzip");
            }
            Compression::Identity => {}
        }
        request = request.header(ACCEPT_ENCODING, "zstd, gzip").header("x-mode", mode);
        let response = self.auth(request).body(body).send().await.map_err(|e| failure("request", &e))?;

        let status = response.status().as_u16();
        let header = |name| response.headers().get(name).and_then(|v| v.to_str().ok()).unwrap_or("").to_string();
        let request_id = header("RequestID");
        let encoding = header("Content-Encoding");
        let raw = response.bytes().await.map_err(|e| failure("read response", &e))?;
        let data = match encoding.trim().to_ascii_lowercase().as_str() {
            "" | "identity" => Ok(raw.to_vec()),
            "gzip" => {
                let mut data = Vec::new();
                flate2::bufread::MultiGzDecoder::new(&raw[..]).read_to_end(&mut data).map(|_| data)
            }
            "zstd" => zstd::stream::decode_all(&raw[..]),
            _ => return Err(Error::local(format!("onestop: unsupported response compression {encoding:?}"))),
        }
        .map_err(|e| failure("decompress response", &e))?;

        let Ok((success, error, mut result)) = decode_json::<T>(&data, provider) else {
            return Err(Error::api(status, &request_id, "invalid JSON response"));
        };
        if status != 200 || !success || !error.is_empty() {
            let message = if error.is_empty() { "request failed" } else { error.as_str() };
            return Err(Error::api(status, &request_id, message));
        }
        result.set_request_id(request_id);
        if result.sleep() > 0 {
            tokio::time::sleep(Duration::from_millis(result.sleep() as u64)).await;
        }
        Ok(result)
    }
}

/// A [`Session`] bound to a [`Client`]; created by [`Client::session`].
#[derive(Debug, Clone)]
pub struct ClientSession {
    pub(crate) client: Client,
    pub(crate) session: Session,
}

impl ClientSession {
    /// The merged identity this session sends.
    pub fn identity(&self) -> &Session {
        &self.session
    }
}

fn decode_json<T: DeserializeOwned + Default>(data: &[u8], provider: &str) -> serde_json::Result<(bool, String, T)> {
    let mut fields = serde_json::from_slice::<Option<Map<String, Value>>>(data)?.unwrap_or_default();
    let mut take = |key: &str| fields.remove(key).unwrap_or(Value::Null);
    let success = serde_json::from_value::<Option<bool>>(take("success"))?.unwrap_or_default();
    let error = serde_json::from_value::<Option<String>>(take("error"))?.unwrap_or_default();
    let result = serde_json::from_value::<Option<T>>(take(&format!("{provider}Response")))?.unwrap_or_default();
    Ok((success, error, result))
}

fn failure(stage: &str, error: &dyn std::error::Error) -> Error {
    let mut message = format!("onestop: {stage}: {error}");
    let mut source = error.source();
    while let Some(cause) = source {
        message = format!("{message}: {cause}");
        source = cause.source();
    }
    Error::local(message)
}
