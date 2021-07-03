use chrono::{Duration, Utc};
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;

use anyhow::Result;

#[derive(Deserialize)]
struct TwitchToken {
    access_token: String,
    expires_in: i64,
}

#[allow(dead_code)]
struct AccessToken {
    access_token: String,
    create_at: chrono::NaiveDateTime,
    expire_at: chrono::NaiveDateTime,
}

pub struct IGDB {
    token: Mutex<AccessToken>,
    client: reqwest::blocking::Client,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IGDBGame {
    pub id: i32,

    #[serde(default)]
    pub cover: Option<Cover>,

    #[serde(default)]
    #[serde(rename = "first_release_date")]
    pub first_release_date: Option<i64>,

    #[serde(default)]
    #[serde(rename = "involved_companies")]
    pub involved_companies: Option<Vec<InvolvedCompany>>,

    pub name: String,

    #[serde(default)]
    pub platforms: Vec<Platform>,

    #[serde(default)]
    #[serde(rename = "release_dates")]
    pub release_dates: Option<Vec<ReleaseDate>>,

    #[serde(default)]
    #[serde(rename = "total_rating")]
    pub total_rating: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    pub id: i64,
    #[serde(rename = "image_id")]
    pub image_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvolvedCompany {
    pub id: i64,
    pub company: Company,
    pub developer: bool,
    pub publisher: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    pub id: i32,
    pub name: String,

    #[serde(default)]
    #[serde(rename = "platform_family")]
    pub platform_family: Option<i32>,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseDate {
    pub id: i32,

    #[serde(default)]
    pub date: Option<i64>,

    pub human: String,
    pub platform: Platform,
}

const API_URL: &str = "https://api.igdb.com/v4";
const TOKEN_API_URL: &str = "https://id.twitch.tv/oauth2/token";
const PC_PLATFORM_ID: i32 = 6;

impl IGDB {
    fn generate_access_token() -> std::result::Result<AccessToken, reqwest::Error> {
        let client_id = env::var("IGDB_CLIENT_ID").expect("IGDB_CLIENT_ID must be set");
        let client_secret = env::var("IGDB_CLIENT_SECRET").expect("IGDB_CLIENT_SECRET must be set");

        let client = reqwest::blocking::Client::new();
        let req_url = format!("{url}?client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials",
            url=TOKEN_API_URL,
            client_id=client_id,
            client_secret=client_secret
        );
        let token = client.post(req_url).send()?.json::<TwitchToken>()?;

        let now_utc = Utc::now().naive_utc();
        Ok(AccessToken {
            access_token: token.access_token,
            create_at: now_utc,
            expire_at: now_utc + Duration::seconds(token.expires_in),
        })
    }

    pub fn new() -> Result<IGDB> {
        let client_id = env::var("IGDB_CLIENT_ID").expect("IGDB_CLIENT_ID must be set");
        let mut headers = header::HeaderMap::new();
        headers.insert("Client-ID", client_id.parse().unwrap());

        let req_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(IGDB {
            token: Mutex::new(AccessToken {
                access_token: String::from(""),
                create_at: Utc::now().naive_utc(),
                expire_at: Utc::now().naive_utc() + Duration::seconds(5),
            }),
            client: req_client,
        })
    }

    fn get_token(&self) {
        let mut token = self.token.lock().unwrap();

        let now_utc = Utc::now().naive_utc();
        let expiry = token.expire_at.signed_duration_since(now_utc).num_seconds();
        log::info!("Token expiring in: {}", expiry);
        if expiry < 5 {
            let new_token = Self::generate_access_token().unwrap();
            *token = new_token;
            log::info!("Will renew");
        }
    }

    pub fn search_games(&self, search_keyword: &str) -> Result<Vec<IGDBGame>> {
        // Refresh token if about to expire
        self.get_token();

        let token = self.token.lock().unwrap();

        let req_url = format!("{url}/games", url = API_URL);
        let req_body = format!(
            "search \"{name_to_search}\"; \
            fields first_release_date, involved_companies.company.name, \
            involved_companies.developer, involved_companies.publisher, \
            name, cover.image_id, parent_game.*, version_parent.*, total_rating, \
            release_dates.date, release_dates.*, release_dates.platform.slug, \
            release_dates.platform.name, release_dates.platform.platform_family, \
            platforms.name, platforms.slug; \
            where version_parent = null & parent_game = null;\
        ",
            name_to_search = search_keyword
        );

        let resp = self
            .client
            .post(req_url)
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", &token.access_token.clone()),
            )
            .body(req_body)
            .send()
            .unwrap();

        // Print the status code
        log::info!("API Status: {}", resp.status().as_u16());
        //log::info!("API Response: {}", resp.text().unwrap());

        let resp_body = resp.json::<Vec<IGDBGame>>().unwrap();

        Ok(resp_body)
    }
}

impl IGDBGame {
    pub fn get_pc_release_date(&self) -> i64 {
        match &self.release_dates {
            None => return 0,
            Some(release_dates) => {
                for date in release_dates.iter() {
                    if date.platform.id == PC_PLATFORM_ID {
                        match date.date {
                            None => return 0,
                            Some(dt) => return dt
                        }
                    }
                }

                return 0
            }
        }
    }
}
