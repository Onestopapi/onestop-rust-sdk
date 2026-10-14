use ring::digest::{digest, SHA256};
use ring::rand::{generate, SystemRandom};

use crate::types::AkamaiSecCptCall;
use crate::Error;

/// Solves the sec-cpt crypto challenge locally; returns the body to POST to `/_sec/verify?provider=<provider>` (crypto or adaptive). No client or API key is needed.
pub fn akamai_sec_cpt_payload(call: &AkamaiSecCptCall) -> Result<String, Error> {
    let Some((prefix, _)) = call.cookie.split_once('~') else {
        return Err(Error::local("akamai: sec_cpt cookie must contain \"~\""));
    };
    if call.token.is_empty() || call.nonce.is_empty() || call.timestamp <= 0 {
        return Err(Error::local("akamai: token, nonce and a positive timestamp are required"));
    }
    if call.difficulty <= 0 || call.count <= 0 {
        return Err(Error::local("akamai: difficulty and count must be positive"));
    }
    let rng = SystemRandom::new();
    let seed = format!("{prefix}{}{}", call.timestamp, call.nonce);
    let mut answers = Vec::with_capacity(call.count as usize);
    for i in 0..call.count {
        let target = call.difficulty + i;
        let base = format!("{seed}{target}");
        loop {
            let bits = u64::from_le_bytes(generate(&rng).unwrap().expose()) >> 11;
            let answer = candidate(bits as f64 / (1u64 << 53) as f64);
            let sum = digest(&SHA256, format!("{base}{answer}").as_bytes());
            let remainder = sum.as_ref().iter().fold(0i64, |acc, &b| ((acc << 8) | i64::from(b)) % target);
            if remainder == 0 {
                answers.push(answer);
                break;
            }
        }
    }
    Ok(format!(r#"{{"token": "{}", "answers": ["{}"]}}"#, call.token, answers.join("\",\"")))
}

/// Formats `value` in [0, 1) the way the browser's `Number.prototype.toString(16)` does.
fn candidate(value: f64) -> String {
    let mut delta = (0.5 * (value.next_up() - value)).max(f64::from_bits(1));
    let mut fraction = value;
    let mut digits: Vec<u8> = Vec::new();
    while fraction >= delta {
        fraction *= 16.0;
        delta *= 16.0;
        let digit = fraction as u8;
        fraction -= f64::from(digit);
        digits.push(digit);
        if (fraction > 0.5 || (fraction == 0.5 && digit & 1 == 1)) && fraction + delta > 1.0 {
            while digits.last() == Some(&15) {
                digits.pop();
            }
            match digits.last_mut() {
                None => return "1".to_string(),
                Some(last) => *last += 1,
            }
            break;
        }
    }
    if digits.is_empty() {
        return "0".to_string();
    }
    let text: String = digits.iter().map(|&d| char::from_digit(u32::from(d), 16).unwrap()).collect();
    format!("0.{text}")
}
