# Changelog

## 0.3.0 (2026-11-27)

- DataDome: add explicit interstitial, captcha page and captcha script URLs.
- Akamai: limit sensor examples to 6 POSTs and distinguish SBSD blocks by `t=`.
- Docs: clarify DataDome URL inputs and Kasada /tl cookies.
- DataDome struct literals need the new fields or `..Default::default()`.

## 0.2.1 (2026-10-19)

### Added

- Imperva (utmvc): `session.imperva_utmvc(&script_url, &script, &cookies).await` returns `Result<onestop::ImpervaResponse, onestop::Error>`; its payload is the `___utmvc` cookie value.

## 0.2.0 (2026-10-20)

### Added

- Imperva (reese84): `session.imperva(&script_url, &script).await` returns `Result<onestop::ImpervaResponse, onestop::Error>`.

### Changed

- Akamai sensor: the pass is `_abck` containing `~0~` or the exact 17-byte `{"success": true}` reply (docs).
- Kasada: each protected request sends the `x-kpsdk-h` from `mfc` and a fresh `x-kpsdk-cd` (docs).
- TLS: use the newest Chrome profile the library has, and keep the User-Agent and `sec-ch-ua` on the latest Chrome (docs).

## 0.1.0 (2026-10-14)

Initial release.
