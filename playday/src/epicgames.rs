use anyhow::Result;
use reqwest::header;
use serde::Deserialize;

pub struct EpicGames {
    http_client: reqwest::blocking::Client,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpicGamesToken {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "refresh_expires")]
    pub refresh_expires: i64,
    #[serde(rename = "refresh_expires_at")]
    pub refresh_expires_at: String,
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_service")]
    pub client_service: String,
    pub display_name: String,
    pub app: String,
    #[serde(rename = "in_app_id")]
    pub in_app_id: String,
    #[serde(rename = "device_id")]
    pub device_id: String,
}

impl EpicGames {
    pub fn new() -> Result<EpicGames> {
        let req_client = reqwest::blocking::Client::builder()
            .build()
            .unwrap();

        Ok(EpicGames {
            http_client: req_client,
        })
    }

    pub fn get_exchange_token(&self, sid: &str) -> Result<String> {

        let mut headers = header::HeaderMap::new();
        headers.insert("X-Epic-Event-Action", "login".parse().unwrap());
        headers.insert("X-Epic-Event-Category", "login".parse().unwrap());
        headers.insert("X-Epic-Strategy-Flags", "".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:86.0) Gecko/20100101 Firefox/86.0".parse().unwrap());

        let req_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();

        let req_url = format!("{url}?sid={sid}",
            url="https://www.epicgames.com/id/api/set-sid",
            sid=sid
        );

        // get first set of cookies (EPIC_BEARER_TOKEN etc.)
        req_client.get(req_url).send().unwrap();

        // get XSRF-TOKEN and EPIC_SESSION_AP cookie
        let resp = req_client.get("https://www.epicgames.com/id/api/csrf").send().unwrap();

        let cookies = resp.cookies().collect::<Vec<_>>();

        let mut xsrf_token: String = "".to_string();
        for cookie in cookies {
            let cookie_name = cookie.name().to_uppercase();
            if cookie_name == "XSRF-TOKEN" {
                xsrf_token = cookie.value().to_string();
            }
        }

        #[derive(Deserialize)]
        struct ExchangeCode {
            code: String,
        }

        let code_resp = req_client.post("https://www.epicgames.com/id/api/exchange/generate")
            .header("X-XSRF-TOKEN", xsrf_token)
            .send().unwrap().json::<ExchangeCode>().unwrap();

        Ok(code_resp.code)
    }

    pub fn get_login_tokens(&self, exchange_code: &str) -> Result<EpicGamesToken> {
        let params = [
            ("grant_type", "exchange_code"),
            ("exchange_code", exchange_code),
            ("token_type", "eg1")
        ];

        let token = self.http_client
            .post("https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token")
            .header("Authorization", "Basic MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y=")
            .form(&params)
            .send().unwrap().json::<EpicGamesToken>().unwrap();

        Ok(token)
    }
}
