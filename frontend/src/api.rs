use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use web_sys::FormData;
use web_sys::RequestCredentials;

pub const BACKEND_URL: &'static str = "http://127.0.0.1:8000";

#[derive(Deserialize)]
struct JsonError<T> {
    error: T,
}

#[derive(PartialEq, Clone, Deserialize, Debug)]
pub struct UserAuth {
    pub id: i32,
    pub email: String,
    pub token: String,
    pub image: Option<String>,
    pub is_admin: bool,
    pub is_staff: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("network error")]
    NetworkError(#[from] gloo_net::Error),
    #[error("{0}")]
    AppError(serde_json::Value),
}

impl ApiError {
    pub fn to_vec_string(&self) -> Vec<String> {
        match self {
            ApiError::NetworkError(err) => vec![format!("network error: {}", err)],
            ApiError::AppError(json) => {
                if let Some(obj) = json.as_object() {
                    obj.iter()
                        .map(|(key, value)| {
                            format!("{}: {}", key, value.as_str().unwrap_or("unknown"))
                        })
                        .collect()
                } else {
                    vec![format!("{}", json)]
                }
            }
        }
    }
}

pub struct ApiRequest(Request);

impl ApiRequest {
    pub fn get(url: impl AsRef<str>) -> Self {
        Self(Request::get(url.as_ref()))
    }

    pub fn post(url: impl AsRef<str>) -> Self {
        Self(Request::post(url.as_ref()))
    }

    pub fn put(url: impl AsRef<str>) -> Self {
        Self(Request::put(url.as_ref()))
    }

    pub fn delete(url: impl AsRef<str>) -> Self {
        Self(Request::delete(url.as_ref()))
    }

    pub fn query<'a, T, V>(self, params: T) -> Self
    where
        T: IntoIterator<Item = (&'a str, V)>,
        V: AsRef<str>,
    {
        Self(self.0.query(params))
    }

    pub fn json(self, json: &impl Serialize) -> Self {
        Self(self.0.json(json).unwrap())
    }

    pub fn body(self, body_data: FormData) -> Self {
        Self(self.0.body(body_data))
    }

    pub fn credentials(self, credentials: RequestCredentials) -> Self {
        Self(self.0.credentials(credentials)) // Add credentials to the request
    }

    pub fn auth(self, auth: Option<&UserAuth>) -> Self {
        if let Some(auth) = auth {
            log::info!("Adding Authorization header with token: {}", auth.token);
            Self(
                self.0
                    .header("Authorization", &format!("Token {}", auth.token)),
            )
        } else {
            self
        }
    }
    pub async fn json_response<T: DeserializeOwned>(self) -> Result<T, ApiError> {
        log::info!("Request: {:?}", self.0);

        let resp = self.0.send().await.map_err(|err| {
            log::error!("Network error: {err:?}");
            ApiError::NetworkError(err)
        })?;

        log::info!("Response: {resp:?}");

        if resp.ok() {
            Ok(resp.json().await.map_err(|err| {
                log::error!("Response json error: {err:?}");
                err
            })?)
        } else {
            let json: JsonError<serde_json::Value> = resp.json().await?;
            Err(ApiError::AppError(json.error))?
        }
    }
}
