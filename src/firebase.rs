use reqwest::{Client, Response};
use serde::Serialize;

pub struct FirebaseError {
    pub message: String,
}

impl FirebaseError {
    fn from_string(message: String) -> FirebaseError {
        FirebaseError { message }
    }
}

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

    pub async fn get(&self, path: &str) -> Result<Response, FirebaseError> {
        let result = self.client.get(self.get_url(path)).send().await;

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(FirebaseError::from_string(e.to_string())),
        }
    }

    pub async fn put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, FirebaseError> {
        let result = self.client.put(self.get_url(path)).json(body).send().await;

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(FirebaseError::from_string(e.to_string())),
        }
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

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(FirebaseError::from_string(e.to_string())),
        }
    }
}

pub fn init_database(project_id: String, token: &str) -> Database {
    let database = Database::new(project_id, token.to_string());

    database
}
