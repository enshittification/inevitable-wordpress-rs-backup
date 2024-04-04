#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub use api_error::*;
pub use login::*;
pub use pages::*;
pub use posts::*;
pub use url::*;

pub mod api_error;
pub mod login;
pub mod pages;
pub mod posts;
pub mod url;

#[derive(uniffi::Object)]
pub struct WPApiHelper {
    site_url: Url,
    authentication: WPAuthentication,
}

#[uniffi::export]
impl WPApiHelper {
    #[uniffi::constructor]
    pub fn new(site_url: String, authentication: WPAuthentication) -> Self {
        let url = Url::parse(site_url.as_str()).unwrap();

        Self {
            site_url: url,
            authentication,
        }
    }

    pub fn raw_request(&self, url: String) -> WPNetworkRequest {
        let mut header_map = HashMap::new();

        match &self.authentication {
            WPAuthentication::AuthorizationHeader { token } => {
                header_map.insert("Authorization".into(), format!("Basic {}", token));
            }
            WPAuthentication::None => (),
        }

        WPNetworkRequest {
            method: RequestMethod::GET,
            url: Url::parse(url.as_str()).unwrap().into(),
            header_map: Some(header_map),
        }
    }

    pub fn post_list_request(&self, params: PostListParams) -> WPNetworkRequest {
        let mut url = self
            .site_url
            .join("/wp-json/wp/v2/posts?context=edit")
            .unwrap();

        let mut header_map = HashMap::new();

        match &self.authentication {
            WPAuthentication::AuthorizationHeader { token } => {
                header_map.insert("Authorization".into(), format!("Basic {}", token));
            }
            WPAuthentication::None => (),
        }

        url.query_pairs_mut()
            .append_pair("page", params.page.to_string().as_str());
        url.query_pairs_mut()
            .append_pair("per_page", params.per_page.to_string().as_str());

        WPNetworkRequest {
            method: RequestMethod::GET,
            url: url.into(),
            header_map: Some(header_map),
        }
    }
}

#[derive(Debug, Clone, uniffi::Enum)]
pub enum WPAuthentication {
    AuthorizationHeader { token: String },
    None,
}

#[derive(uniffi::Enum)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
}

#[derive(uniffi::Record)]
pub struct WPNetworkRequest {
    pub method: RequestMethod,
    pub url: String,
    // TODO: We probably want to implement a specific type for these headers instead of using a
    // regular HashMap.
    //
    // It could be something similar to `reqwest`'s [`header`](https://docs.rs/reqwest/latest/reqwest/header/index.html)
    // module.
    pub header_map: Option<HashMap<String, String>>,
}

#[derive(Debug, uniffi::Record)]
pub struct WPNetworkResponse {
    pub body: Vec<u8>,
    pub status_code: u16,
    // TODO: We probably want to implement a specific type for these headers instead of using a
    // regular HashMap.
    //
    // It could be something similar to `reqwest`'s [`header`](https://docs.rs/reqwest/latest/reqwest/header/index.html)
    // module.
    pub header_map: Option<HashMap<String, String>>,
}

impl WPNetworkResponse {
    pub fn get_link_header(&self, name: &str) -> Option<Url> {
        if let Some(headers) = self.header_map.clone() {
            // TODO: This is inefficient
            if headers.contains_key("Link") {
                if let Ok(res) = parse_link_header::parse_with_rel(&headers["Link"]) {
                    if let Some(next) = res.get(name) {
                        if let Ok(url) = Url::parse(next.raw_uri.as_str()) {
                            return Some(url);
                        }
                    }
                }
            }
        }

        None
    }

    fn content_type(&self) -> Option<&String> {
        // TODO: We should make this case insensitive, which will happen along with `header_map` refactor.
        self.header_map
            .as_ref()
            .and_then(|headers| headers.get("Content-Type"))
    }
}

#[uniffi::export]
pub fn parse_post_list_response(
    response: WPNetworkResponse,
) -> Result<PostListResponse, WPApiError> {
    // TODO: Further parse the response body to include error message
    // TODO: Lots of unwraps to get a basic setup working
    if (400..500).contains(&response.status_code) {
        let is_json_response = response
            .content_type()
            .map(|t| t.starts_with("application/json"))
            .unwrap_or(false);
        return if is_json_response {
            match serde_json::from_slice(&response.body) {
                Ok(error) => Err(WPApiError::ClientError {
                    status_code: response.status_code,
                    error,
                }),
                Err(err) => Err(WPApiError::ParsingError {
                    reason: err.to_string(),
                    response,
                }),
            }
        } else {
            Err(WPApiError::ClientError {
                status_code: response.status_code,
                error: None,
            })
        };
    }

    if (500..600).contains(&response.status_code) {
        return Err(WPApiError::ServerError {
            status_code: response.status_code,
        });
    }

    let parsed: Result<Vec<PostObject>, _> = serde_json::from_slice(&response.body);
    if let Err(err) = parsed {
        return Err(WPApiError::ParsingError {
            reason: err.to_string(),
            response,
        });
    }

    let post_list: Vec<PostObject> = parsed.unwrap();

    let mut next_page: Option<String> = None;

    if let Some(link_header) = response.get_link_header("next") {
        next_page = Some(link_header.to_string())
    }

    Ok(PostListResponse {
        post_list: Some(post_list),
        next_page,
    })
}

#[uniffi::export]
pub fn parse_api_details_response(response: WPNetworkResponse) -> Result<WPAPIDetails, WPApiError> {
    let api_details =
        serde_json::from_slice(&response.body).map_err(|err| WPApiError::ParsingError {
            reason: err.to_string(),
            response,
        })?;

    Ok(api_details)
}

// TODO: Figure out why we can't expose this method on `WPNetworkResponse` via UniFFI
#[uniffi::export]
pub fn get_link_header(response: &WPNetworkResponse, name: &str) -> Option<WPRestAPIURL> {
    if let Some(url) = response.get_link_header(name) {
        return Some(url.into());
    }

    None
}

uniffi::setup_scaffolding!();
