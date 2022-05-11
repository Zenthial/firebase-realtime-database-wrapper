#![allow(dead_code)]

use gcp_auth::{AuthenticationManager, CustomServiceAccount, Token};
use reqwest::{Client, Response};
use serde::Serialize;
use std::error::Error;
use std::path::PathBuf;

pub struct FirebaseError {
    pub message: String,
}

impl FirebaseError {
    fn from_string(message: String) -> FirebaseError {
        FirebaseError { message }
    }
}

#[derive(Clone)]
pub struct Database {
    project_id: String,
    access_token: String,
    client: Client,
}

impl Database {
    fn new(project_id: String, access_token: String) -> Database {
        let client = Client::new();

        Database {
            project_id,
            access_token,
            client,
        }
    }

    fn get_url(&self, path: &str) -> String {
        format!(
            "https://{}.firebaseio.com/{}.json?access_token={}",
            self.project_id, path, self.access_token
        )
    }

    fn parse_result(
        &self,
        result: Result<Response, reqwest::Error>,
    ) -> Result<Response, FirebaseError> {
        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(FirebaseError::from_string(e.to_string())),
        }
    }

    pub async fn get(&self, path: &str) -> Result<Response, FirebaseError> {
        let result = self.client.get(self.get_url(path)).send().await;

        self.parse_result(result)
    }

    pub async fn delete(&self, path: &str) -> Result<Response, FirebaseError> {
        let result = self.client.delete(self.get_url(path)).send().await;

        self.parse_result(result)
    }

    pub async fn put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, FirebaseError> {
        let result = self.client.put(self.get_url(path)).json(body).send().await;

        self.parse_result(result)
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, FirebaseError> {
        let result = self.client.post(self.get_url(path)).json(body).send().await;

        self.parse_result(result)
    }

    pub async fn update<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, FirebaseError> {
        let result = self
            .client
            .patch(self.get_url(path))
            .json(body)
            .send()
            .await;

        self.parse_result(result)
    }
}

pub fn create_database(project_id: &str, token: &str) -> Database {
    let database = Database::new(project_id.to_string(), token.to_string());

    database
}

pub async fn get_oauth_token(path: &str) -> Result<Token, Box<dyn Error>> {
    // `credentials_path` variable is the path for the credentials `.json` file.
    let credentials_path = PathBuf::from(path);
    let service_account = CustomServiceAccount::from_file(credentials_path)?;
    let authentication_manager = AuthenticationManager::from(service_account);
    let scopes = &[
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/firebase.database",
    ];
    let token = authentication_manager.get_token(scopes).await?;

    Ok(token)
}
