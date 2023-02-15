use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

use crate::domain::NewUser;

pub struct PasswordlessClient {
    http_client: Client,
    base_url: String,
    authorization_secret: Secret<String>,
}
impl PasswordlessClient {
    pub fn new(
        base_url: String,
        authorization_secret: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        Self {
            http_client: Client::builder().timeout(timeout).build().unwrap(),
            base_url,
            authorization_secret,
        }
    }

    pub async fn try_register(&self, user_info: &NewUser, user_id: Uuid) -> Result<String, String> {
        let url = format!("{}/register/token", self.base_url);

        let request = match self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("ApiSecret", self.authorization_secret.expose_secret())
            .json(&serde_json::json!({
                "userId": user_id,
                "username": user_info.username,
                "displayName": user_info.display_name,
                "authType": "cross-platform"
            }))
            .send()
            .await
        {
            Ok(request) => {
                if request.status().is_success() {
                    Ok(request.text().await.unwrap())
                } else {
                    Err(request.text().await.unwrap())
                }
            }
            Err(_) => Err("Something went wrong!".to_string()),
        };
        request
    }

    pub async fn try_login(&self, token: &str) -> Result<String, String> {
        let url = format!("{}/signin/verify", self.base_url);

        let request = match self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("ApiSecret", self.authorization_secret.expose_secret())
            .json(token)
            .send()
            .await
        {
            Ok(request) => {
                if request.status().is_success() {
                    Ok(request.text().await.unwrap())
                } else {
                    Err(request.text().await.unwrap())
                }
            }
            Err(_) => Err("Something went wrong!".to_string()),
        };
        request
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::turnstile_client::PasswordlessClient;
//     use claims::{assert_err, assert_ok};
//     use secrecy::Secret;
//     use wiremock::matchers::{any, method, path, query_param};
//     use wiremock::Request;
//     use wiremock::{Mock, MockServer, ResponseTemplate};

//     struct TryValidationResponseMatcher;

//     impl wiremock::Match for TryValidationResponseMatcher {
//         fn matches(&self, request: &Request) -> bool {
//             let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);

//             if let Ok(body) = result {
//                 dbg!(&body);
//                 body.get("secret").is_some()
//                     && body.get("response").is_some()
//             } else {
//                 false
//             }
//         }
//     }

//     #[derive(serde::Serialize)]
//     struct TryValidationResponseTemplate {
//         success: bool,
//     }

//     /// Get a test instance of `PasswordlessClient`.
//     fn turnstile_client(base_url: String) -> PasswordlessClient {
//         PasswordlessClient::new(
//             base_url,
//             Secret::new("some-secret".to_string()),
//             std::time::Duration::from_millis(200),
//         )
//     }

//     #[tokio::test]
//     async fn try_validation_sends_expected_request() {
//         let mock_server = MockServer::start().await;
//         let turnstile_client = turnstile_client(mock_server.uri());

//         Mock::given(path("/siteverify"))
//             .and(method("POST"))
//             .and(query_param("secret", "some-secret"))
//             .and(query_param("response", "some-response"))
//             .respond_with(ResponseTemplate::new(200).set_body_json(TryValidationResponseTemplate {
//                 success: true,
//             }))
//             .expect(1)
//             .mount(&mock_server)
//             .await;

//         let _ = turnstile_client
//             .try_validation("some-response", "0.0.0.0")
//             .await;
//     }

//     #[tokio::test]
//     async fn try_validation_succeeds_if_the_server_returns_200() {
//         let mock_server = MockServer::start().await;
//         let turnstile_client = turnstile_client(mock_server.uri());

//         Mock::given(any())
//             .respond_with(ResponseTemplate::new(200).set_body_json(TryValidationResponseTemplate {
//                 success: true,
//             }))
//             .expect(1)
//             .mount(&mock_server)
//             .await;

//         let outcome = turnstile_client
//             .try_validation("some-response", "0.0.0.0")
//             .await;

//         assert_ok!(outcome);
//     }

//     #[tokio::test]
//     async fn try_validation_fails_if_the_server_returns_200_and_a_fail() {
//         let mock_server = MockServer::start().await;
//         let turnstile_client = turnstile_client(mock_server.uri());

//         Mock::given(any())
//             .respond_with(ResponseTemplate::new(200).set_body_json(TryValidationResponseTemplate {
//                 success: false,
//             }))
//             .expect(1)
//             .mount(&mock_server)
//             .await;

//         let outcome = turnstile_client
//             .try_validation("some-response", "0.0.0.0")
//             .await;

//         assert_err!(outcome);
//     }

//     // #[tokio::test]
//     // async fn try_validation_fails_if_the_server_returns_500() {
//     //     let mock_server = MockServer::start().await;
//     //     let turnstile_client = turnstile_client(mock_server.uri());

//     //     Mock::given(any())
//     //         .respond_with(ResponseTemplate::new(500))
//     //         .expect(1)
//     //         .mount(&mock_server)
//     //         .await;

//     //     let outcome = turnstile_client
//     //         .try_validation("some-response", "0.0.0.0")
//     //         .await;

//     //     assert_err!(outcome);
//     // }

//     #[tokio::test]
//     async fn try_validation_times_out_if_server_takes_too_long() {
//         let mock_server = MockServer::start().await;
//         let turnstile_client = turnstile_client(mock_server.uri());

//         let reponse = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));

//         Mock::given(any())
//             .respond_with(reponse)
//             .expect(1)
//             .mount(&mock_server)
//             .await;

//         let outcome = turnstile_client
//             .try_validation("some-response", "0.0.0.0")
//             .await;

//         assert_err!(outcome);
//     }
// }
