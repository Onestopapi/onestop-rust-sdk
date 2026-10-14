//! Client for the OneStop Antibot API.
//!
//! Build a [`Client`] from a [`Config`], bind a browser identity with
//! [`Client::session`], then call one method per product on the returned
//! [`ClientSession`]. Every method returns its typed response, which carries
//! the API `request_id`, or an [`Error`]. [`kasada_cd_header`] and
//! [`akamai_sec_cpt_payload`] run locally.

mod akamai;
mod client;
mod errors;
mod kasada;
mod session;
mod types;
mod wire;

pub use akamai::akamai_sec_cpt_payload;
pub use client::{Client, ClientSession, Config, DEFAULT_ENDPOINT};
pub use errors::Error;
pub use kasada::kasada_cd_header;
pub use reqwest;
pub use types::{
    AkamaiResponse, AkamaiSecCptCall, AkamaiSensorCall, BmpCall, BmpResponse, CloudflareCall, CloudflareHttpRequest, CloudflareResponse,
    DatadomeInterstitialCall, DatadomeInterstitialResponse, DatadomeResponse, DatadomeSliderCall, DatadomeSliderResponse, DatadomeTagsCall,
    DatadomeTagsResponse, KasadaCdCall, KasadaPScriptResponse, KasadaResponse, PixelCall, PixelResponse, SbsdCall, SbsdResponse, Session,
};
