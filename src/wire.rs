use std::io;

use serde::Serialize;

use crate::types::Session;

pub(crate) fn encode<T: Serialize>(value: &T) -> serde_json::Result<Vec<u8>> {
    let mut out = Vec::new();
    value.serialize(&mut serde_json::Serializer::with_formatter(&mut out, GoFormatter))?;
    out.push(b'\n');
    Ok(out)
}

// Go's encoder escapes U+2028 and U+2029 even with HTML escaping off.
struct GoFormatter;

impl serde_json::ser::Formatter for GoFormatter {
    fn write_string_fragment<W: ?Sized + io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
        let mut start = 0;
        for (i, c) in fragment.char_indices() {
            if c == '\u{2028}' || c == '\u{2029}' {
                writer.write_all(&fragment.as_bytes()[start..i])?;
                write!(writer, "\\u{:04x}", c as u32)?;
                start = i + c.len_utf8();
            }
        }
        writer.write_all(&fragment.as_bytes()[start..])
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Akamai<'a> {
    #[serde(skip_serializing_if = "str::is_empty")]
    pub referer: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub site: &'a str,
    pub abck: &'a str,
    pub bmsz: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub user_agent: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub accept_language: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub ip: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub script: &'a str,
    pub script_url: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub context: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Sbsd<'a> {
    #[serde(skip_serializing_if = "str::is_empty")]
    pub referer: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub site: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub user_agent: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub accept_language: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub ip: &'a str,
    pub uuid: &'a str,
    pub sbsd_cookie: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub script: &'a str,
    pub mode: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub context: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KasadaPScript<'a> {
    pub referer: &'a str,
    pub site: &'a str,
    pub user_agent: &'a str,
    pub accept_language: &'a str,
    pub ip: &'a str,
    pub script: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KasadaTl<'a> {
    #[serde(skip_serializing_if = "str::is_empty")]
    pub referer: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub site: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub user_agent: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub accept_language: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub ip: &'a str,
    pub ips_url: &'a str,
    pub script: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Identity<'a, C> {
    #[serde(skip_serializing_if = "str::is_empty")]
    site: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    user_agent: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    referer: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    accept_language: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    ip: &'a str,
    #[serde(flatten)]
    call: &'a C,
}

impl<'a, C> Identity<'a, C> {
    pub fn new(s: &'a Session, call: &'a C) -> Self {
        Self { site: &s.site, user_agent: &s.user_agent, referer: &s.referer, accept_language: &s.accept_language, ip: &s.ip, call }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DatadomeIdentity<'a, C> {
    #[serde(skip_serializing_if = "str::is_empty")]
    site: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    user_agent: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    accept_language: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    ip: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    referer: &'a str,
    #[serde(flatten)]
    call: &'a C,
}

impl<'a, C> DatadomeIdentity<'a, C> {
    pub fn new(s: &'a Session, call: &'a C) -> Self {
        Self { site: &s.site, user_agent: &s.user_agent, accept_language: &s.accept_language, ip: &s.ip, referer: &s.referer, call }
    }
}
