use gcp_auth::Token;
use gcp_auth::{AuthenticationManager, CustomServiceAccount};
use std::error::Error;
use std::path::PathBuf;

pub async fn get_token() -> Result<Token, Box<dyn Error>> {
    // `credentials_path` variable is the path for the credentials `.json` file.
    let credentials_path = PathBuf::from("wave-mainframe-key.json");
    let service_account = CustomServiceAccount::from_file(credentials_path)?;
    let authentication_manager = AuthenticationManager::from(service_account);
    let scopes = &[
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/firebase.database",
    ];
    let token = authentication_manager.get_token(scopes).await?;

    Ok(token)
}
