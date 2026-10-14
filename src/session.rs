use crate::client::ClientSession;
use crate::types::*;
use crate::wire;
use crate::Error;

impl ClientSession {
    pub async fn akamai_sensor(&self, call: &AkamaiSensorCall) -> Result<AkamaiResponse, Error> {
        let s = &self.session;
        let input = wire::Akamai {
            referer: &s.referer,
            site: &s.site,
            abck: &call.abck,
            bmsz: &call.bmsz,
            user_agent: &s.user_agent,
            accept_language: &s.accept_language,
            ip: &s.ip,
            script: &call.script,
            script_url: &call.script_url,
            context: &call.context,
        };
        self.client.generate("Akamai", "akamai", &input).await
    }

    pub async fn sbsd_payload(&self, call: &SbsdCall) -> Result<SbsdResponse, Error> {
        let s = &self.session;
        let input = wire::Sbsd {
            referer: &s.referer,
            site: &s.site,
            user_agent: &s.user_agent,
            accept_language: &s.accept_language,
            ip: &s.ip,
            uuid: &call.uuid,
            sbsd_cookie: &call.sbsd_cookie,
            script: &call.script,
            mode: &call.mode,
            context: &call.context,
        };
        self.client.generate("Sbsd", "sbsd", &input).await
    }

    pub async fn pixel(&self, call: &PixelCall) -> Result<PixelResponse, Error> {
        self.client.generate("Akamai", "pixel", &wire::Identity::new(&self.session, call)).await
    }

    /// Akamai BMP (mobile SDK) payload; no browser identity is sent.
    pub async fn bmp(&self, call: &BmpCall) -> Result<BmpResponse, Error> {
        self.client.generate("Akamai", "bmp", call).await
    }

    pub async fn datadome_tags(&self, call: &DatadomeTagsCall) -> Result<DatadomeTagsResponse, Error> {
        let input = wire::DatadomeIdentity::new(&self.session, call);
        self.client.generate("DatadomeTags", "datadomeTags", &input).await
    }

    pub async fn datadome_interstitial(&self, call: &DatadomeInterstitialCall) -> Result<DatadomeInterstitialResponse, Error> {
        let input = wire::DatadomeIdentity::new(&self.session, call);
        self.client.generate("DatadomeInterstitial", "datadomeInterstitial", &input).await
    }

    /// Returns the check request's query string as `payload`.
    pub async fn datadome_slider(&self, call: &DatadomeSliderCall) -> Result<DatadomeSliderResponse, Error> {
        let input = wire::DatadomeIdentity::new(&self.session, call);
        self.client.generate("DatadomeSlider", "datadomeSlider", &input).await
    }

    /// Runs one Cloudflare step; the API detects the challenge type.
    pub async fn cloudflare_step(&self, call: &CloudflareCall) -> Result<CloudflareResponse, Error> {
        self.cloudflare("Cloudflare", call).await
    }

    pub async fn cloudflare_turnstile(&self, call: &CloudflareCall) -> Result<CloudflareResponse, Error> {
        self.cloudflare("CloudflareTurnstile", call).await
    }

    pub async fn cloudflare_waf(&self, call: &CloudflareCall) -> Result<CloudflareResponse, Error> {
        self.cloudflare("CloudflareWaf", call).await
    }

    pub async fn cloudflare_jsd(&self, call: &CloudflareCall) -> Result<CloudflareResponse, Error> {
        self.cloudflare("CloudflareJsd", call).await
    }

    pub async fn cloudflare_precursor(&self, call: &CloudflareCall) -> Result<CloudflareResponse, Error> {
        self.cloudflare("CloudflarePrecursor", call).await
    }

    async fn cloudflare(&self, mode: &str, call: &CloudflareCall) -> Result<CloudflareResponse, Error> {
        self.client.generate(mode, "cloudflare", &wire::Identity::new(&self.session, call)).await
    }

    /// Extracts the version and CD inputs from raw p.js source.
    pub async fn kasada_pscript(&self, script: &str) -> Result<KasadaPScriptResponse, Error> {
        let s = &self.session;
        let input = wire::KasadaPScript {
            referer: &s.referer,
            site: &s.site,
            user_agent: &s.user_agent,
            accept_language: &s.accept_language,
            ip: &s.ip,
            script,
        };
        self.client.generate("Kasada", "kasadaPscript", &input).await
    }

    /// Generates the TL body from the ips.js URL and raw source.
    pub async fn kasada_tl(&self, script_url: &str, script: &str) -> Result<KasadaResponse, Error> {
        let s = &self.session;
        let input = wire::KasadaTl {
            referer: &s.referer,
            site: &s.site,
            user_agent: &s.user_agent,
            accept_language: &s.accept_language,
            ip: &s.ip,
            ips_url: script_url,
            script,
        };
        self.client.generate("Kasada", "kasada", &input).await
    }
}
