use std::time::{SystemTime, UNIX_EPOCH};

use ring::digest::{digest, SHA256};
use ring::rand::{generate, SystemRandom};
use serde::{Serialize, Serializer};

use crate::types::{KasadaCdCall, KasadaPScriptResponse};
use crate::Error;

impl KasadaPScriptResponse {
    /// Builds the desktop `x-kpsdk-cd` header from the /tl response's `x-kpsdk-ct` value and `x-kpsdk-st` header (ms).
    /// Use [`kasada_cd_header`] for mobile.
    pub fn cd_header(&self, ct: &str, st: i64) -> Result<String, Error> {
        kasada_cd_header(&KasadaCdCall {
            st,
            rst: now_ms().max(st),
            mobile: false,
            ct: ct.to_string(),
            crypto_customer_key: self.crypto_customer_key.clone(),
            seed_suffix: self.seed_suffix.clone(),
            difficulty: self.difficulty,
            subchallenge_count: self.subchallenge_count,
        })
    }
}

/// Computes the `x-kpsdk-cd` header locally. No client or API key is needed.
pub fn kasada_cd_header(call: &KasadaCdCall) -> Result<String, Error> {
    if call.st <= 0 || call.rst < call.st {
        return Err(Error::local("kasada: require positive ST and RST >= ST"));
    }
    if call.difficulty <= 0 || call.subchallenge_count <= 0 {
        return Err(Error::local("kasada: difficulty and subchallenge count must be positive"));
    }
    let mut input = b"tp-v2-input".to_vec();
    if !call.mobile {
        if call.ct.len() < 16 {
            return Err(Error::local("kasada: desktop CT must contain at least 16 bytes"));
        }
        input.extend_from_slice(&call.ct.as_bytes()[..16]);
    }

    let rng = SystemRandom::new();
    let id = hex(&generate::<[u8; 16]>(&rng).unwrap().expose());
    let jitter = u32::from_le_bytes(generate(&rng).unwrap().expose()) % 41 + 10;
    let fraction = f64::from(u32::from_le_bytes(generate(&rng).unwrap().expose())) / f64::from(u32::MAX);
    let work_time = now_ms() - i64::from(jitter);

    input.extend_from_slice(format!(", {work_time}, {id}").as_bytes());
    for extra in [&call.crypto_customer_key, &call.seed_suffix] {
        if !extra.is_empty() {
            input.extend_from_slice(b", ");
            input.extend_from_slice(extra.as_bytes());
        }
    }

    let mut hash = sha256_hex(&input);
    let target = call.difficulty as f64 / call.subchallenge_count as f64;
    let mut answers = Vec::with_capacity(call.subchallenge_count as usize);
    for _ in 0..call.subchallenge_count {
        for nonce in 1i64.. {
            let next = sha256_hex(format!("{nonce}, {hash}").as_bytes());
            let prefix = u64::from_str_radix(&next[..13], 16).unwrap();
            if 0x10000000000000u64 as f64 / (prefix + 1) as f64 >= target {
                answers.push(nonce);
                hash = next;
                break;
            }
        }
    }
    let duration = ((0.5 + fraction * 1.5) * 10.0).round() / 10.0;
    let (st, rst, d) = (call.st, call.rst, call.rst - call.st);
    let header = if call.mobile {
        serde_json::to_string(&CdMobile { work_time, rst, duration, id, st, answers, d })
    } else {
        serde_json::to_string(&CdDesktop { work_time, id, answers, duration, d, st, rst })
    };
    Ok(header.unwrap())
}

// The x-kpsdk-cd header JSON; field order is the wire order.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CdMobile {
    work_time: i64,
    rst: i64,
    #[serde(serialize_with = "go_float")]
    duration: f64,
    id: String,
    st: i64,
    answers: Vec<i64>,
    d: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CdDesktop {
    work_time: i64,
    id: String,
    answers: Vec<i64>,
    #[serde(serialize_with = "go_float")]
    duration: f64,
    d: i64,
    st: i64,
    rst: i64,
}

// Go's json.Marshal writes a whole float64 without ".0".
fn go_float<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
    if value.fract() == 0.0 {
        serializer.serialize_i64(*value as i64)
    } else {
        serializer.serialize_f64(*value)
    }
}

fn sha256_hex(data: &[u8]) -> String {
    hex(digest(&SHA256, data).as_ref())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn now_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}
