use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};

/// Browser identity sent with every call. Pass it to [`crate::Client::session`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Session {
    /// Absolute page URL.
    pub site: String,
    /// HTTP referer of the page request.
    pub referer: String,
    pub user_agent: String,
    pub accept_language: String,
    /// Public IP the target observes, see [`crate::Client::get_ip`].
    pub ip: String,
}

/// Inputs for [`crate::ClientSession::akamai_sensor`]. Send `script` on the first call and `context`, without `script`, on every
/// later call.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AkamaiSensorCall {
    pub abck: String,
    pub bmsz: String,
    pub script: String,
    pub script_url: String,
    pub context: String,
}

/// Inputs for [`crate::ClientSession::sbsd_payload`]. `mode` is `f` (fingerprint, with `script`) or `t` (event, with `context`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SbsdCall {
    pub uuid: String,
    pub sbsd_cookie: String,
    pub script: String,
    pub mode: String,
    pub context: String,
}

/// Inputs for [`crate::ClientSession::pixel`]: raw page HTML and Pixel script.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PixelCall {
    pub html: String,
    pub script: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub context: String,
}

/// Android/iOS SDK inputs for [`crate::ClientSession::bmp`]; no browser identity is sent.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct BmpCall {
    pub bundle_id: String,
    pub version: String,
    pub device: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub nonce: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub checksum: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub server_side_signal: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<i64>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ios_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub uuid: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub real_values: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub script: String,
}

/// Inputs for [`crate::ClientSession::datadome_tags`]. `event` is `ch` or `le`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DatadomeTagsCall {
    pub event: String,
    pub site_key: String,
    pub version: String,
    pub cookie: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub context: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DatadomeInterstitialCall {
    pub script: String,
    pub html: String,
    pub page_response: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub context: String,
}

/// Inputs for [`crate::ClientSession::datadome_slider`]. `background_image` and `puzzle_piece` are base64.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DatadomeSliderCall {
    pub script: String,
    pub html: String,
    pub page_response: String,
    pub background_image: String,
    pub puzzle_piece: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub context: String,
}

/// Inputs for the Cloudflare methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CloudflareCall {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub html: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub script: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub script_url: String,
    /// The target's reply: a JSON string or any JSON value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub context: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ray_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bytecode_ray_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub iframe_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_status: Option<i64>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub c_data: String,
}

/// Inputs for [`crate::akamai_sec_cpt_payload`]: the `sec_cpt` cookie value and the challenge JSON's fields.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AkamaiSecCptCall {
    pub cookie: String,
    pub token: String,
    pub nonce: String,
    pub timestamp: i64,
    pub difficulty: i64,
    pub count: i64,
}

/// Inputs for [`crate::kasada_cd_header`]. `crypto_customer_key`, `seed_suffix`, `difficulty` and `subchallenge_count` come from
/// [`crate::ClientSession::kasada_pscript`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct KasadaCdCall {
    pub st: i64,
    pub rst: i64,
    pub mobile: bool,
    pub ct: String,
    pub crypto_customer_key: String,
    pub seed_suffix: String,
    pub difficulty: i64,
    pub subchallenge_count: i64,
}

pub(crate) fn nullable<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

pub(crate) trait Meta {
    fn set_request_id(&mut self, request_id: String);
    fn sleep(&self) -> i64;
}

macro_rules! response {
    ($(#[$meta:meta])* $name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $(#[serde(default, deserialize_with = "nullable")] pub $field: $ty,)*
            /// Wait in milliseconds the API asked for; the SDK has already waited it before returning.
            #[serde(default, deserialize_with = "nullable")]
            pub sleep: i64,
            /// API request ID for support.
            #[serde(skip)]
            pub request_id: String,
        }

        impl Meta for $name {
            fn set_request_id(&mut self, request_id: String) {
                self.request_id = request_id;
            }

            fn sleep(&self) -> i64 {
                self.sleep
            }
        }
    };
}

response!(
    /// Returned by `akamai_sensor`, `pixel` and `bmp`.
    AkamaiResponse { payload: String, context: String }
);

response!(SbsdResponse { payload: String, context: String });

response!(
    /// Returned by the DataDome methods. `payload` is already encoded.
    DatadomeResponse { payload: String, context: String }
);

response!(
    /// Values extracted from p.js; the crypto fields feed [`KasadaPScriptResponse::cd_header`].
    KasadaPScriptResponse {
        version: String,
        headers: BTreeMap<String, String>,
        difficulty: i64,
        subchallenge_count: i64,
        seed_suffix: String,
        crypto_customer_key: String,
        cache_lifetime_seconds: i64,
    }
);

response!(
    /// The base64 TL body and the headers to send with it.
    KasadaResponse { payload: String, headers: BTreeMap<String, String> }
);

response!(
    /// The target requests for one challenge step.
    CloudflareResponse {
        requests: Vec<CloudflareHttpRequest>,
        token: String,
        context: String,
    }
);

/// One target request; `purpose` is `challenge`, `iframe`, `image`, `pat` or `form`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareHttpRequest {
    #[serde(default, deserialize_with = "nullable")]
    pub purpose: String,
    #[serde(default, deserialize_with = "nullable")]
    pub method: String,
    #[serde(default, deserialize_with = "nullable")]
    pub url: String,
    #[serde(default, deserialize_with = "nullable")]
    pub headers: BTreeMap<String, String>,
    #[serde(default, deserialize_with = "nullable")]
    pub body: String,
}

/// `payload` is the complete form body for the Pixel POST.
pub type PixelResponse = AkamaiResponse;
pub type BmpResponse = AkamaiResponse;
pub type DatadomeTagsResponse = DatadomeResponse;
pub type DatadomeInterstitialResponse = DatadomeResponse;
pub type DatadomeSliderResponse = DatadomeResponse;
