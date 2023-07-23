#![allow(dead_code)]

use gcp_auth::{AuthenticationManager, CustomServiceAccount};
use reqwest::header::{HeaderMap, CONNECTION};
use reqwest::{Client, Response};
use serde::Serialize;
use std::path::PathBuf;

const SCOPES: &[&str] = &[
    "https://www.googleapis.com/auth/userinfo.email",
    "https://www.googleapis.com/auth/firebase.database",
];

#[derive(Debug)]
pub enum FirebaseError {
    GcpAuthError(gcp_auth::Error),
    ReqwestError(reqwest::Error),
}

impl ToString for FirebaseError {
    fn to_string(&self) -> String {
        match self {
            FirebaseError::GcpAuthError(e) => e.to_string(),
            FirebaseError::ReqwestError(e) => e.to_string(),
        }
    }
}

pub struct Database {
    project_id: String,
    manager: AuthenticationManager,
    client: Client,
}

impl Database {
    pub fn new(project_id: &str, manager: AuthenticationManager) -> Self {
        let client = Client::new();

        Database {
            project_id: project_id.to_string(),
            manager,
            client,
        }
    }

    pub fn from_path(project_id: &str, path: &str) -> Result<Self, gcp_auth::Error> {
        // `credentials_path` variable is the path for the credentials `.json` file.
        let credentials_path = PathBuf::from(path);
        let service_account = CustomServiceAccount::from_file(credentials_path)?;
        let authentication_manager = AuthenticationManager::from(service_account);

        Ok(Self::new(project_id, authentication_manager))
    }

    async fn get_token(&self) -> Result<String, FirebaseError> {
        let token_result = self.manager.get_token(SCOPES).await;

        match token_result {
            Ok(token) => Ok(token.as_str().to_string()),
            Err(e) => Err(FirebaseError::GcpAuthError(e)),
        }
    }

    async fn get_url(&self, path: &str) -> Result<String, FirebaseError> {
        let token = self.get_token().await;

        match token {
            Ok(tok) => Ok(format!(
                "https://{}.firebaseio.com/{}.json?access_token={}",
                self.project_id, path, tok
            )),
            Err(e) => Err(e),
        }
    }

    fn parse_result(
        &self,
        result: Result<Response, reqwest::Error>,
    ) -> Result<Response, FirebaseError> {
        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(FirebaseError::ReqwestError(e)),
        }
    }

    fn get_header_map(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.insert(CONNECTION, "close".parse().unwrap());

        header_map
    }

    pub async fn get(&self, path: &str) -> Result<Response, FirebaseError> {
        let url_result = self.get_url(path).await;

        match url_result {
            Ok(url) => {
                let result = self
                    .client
                    .get(url)
                    .headers(self.get_header_map())
                    .send()
                    .await;

                self.parse_result(result)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&self, path: &str) -> Result<Response, FirebaseError> {
        let url_result = self.get_url(path).await;

        match url_result {
            Ok(url) => {
                let result = self
                    .client
                    .delete(url)
                    .headers(self.get_header_map())
                    .send()
                    .await;

                self.parse_result(result)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, FirebaseError> {
        let url_result = self.get_url(path).await;

        match url_result {
            Ok(url) => {
                let result = self
                    .client
                    .put(url)
                    .headers(self.get_header_map())
                    .json(body)
                    .send()
                    .await;

                self.parse_result(result)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, FirebaseError> {
        let url_result = self.get_url(path).await;

        match url_result {
            Ok(url) => {
                let result = self
                    .client
                    .post(url)
                    .headers(self.get_header_map())
                    .json(body)
                    .send()
                    .await;

                self.parse_result(result)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, FirebaseError> {
        let url_result = self.get_url(path).await;

        match url_result {
            Ok(url) => {
                let result = self
                    .client
                    .patch(url)
                    .headers(self.get_header_map())
                    .json(body)
                    .send()
                    .await;

                self.parse_result(result)
            }
            Err(e) => Err(e),
        }
    }
}

unsafe impl Send for Database {}
