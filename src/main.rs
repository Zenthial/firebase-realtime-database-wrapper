mod auth;
mod firebase;

use std::collections::HashMap;

use auth::get_token;
use firebase::init_database;

#[tokio::main]
async fn main() {
    let token_result = get_token().await;
    // println!("{:?}", token_result);

    if let Ok(token) = token_result {
        // println!("{}", token.as_str());
        let database = init_database(String::from("wave-mainframe-default-rtdb"), token.as_str());
        match database.get("users/tom").await {
            Ok(response) => {
                println!("{}", response.text().await.unwrap());
            }
            Err(e) => {
                panic!("{}", e.message);
            }
        }

        let put_result = database
            .put(
                "users/joe2",
                &HashMap::from([("first_name", "Joe"), ("last_name", "Mama")]),
            )
            .await;

        match put_result {
            Ok(response) => {
                println!("{}", response.text().await.unwrap());
            }
            Err(e) => {
                panic!("{}", e.message);
            }
        }

        let update_result = database
            .update("users/joe", &HashMap::from([("last_name", 3)]))
            .await;

        match update_result {
            Ok(response) => {
                println!("{}", response.text().await.unwrap());
            }
            Err(e) => {
                panic!("{}", e.message);
            }
        }

        let delete_result = database.delete("users/joe2").await;

        match delete_result {
            Ok(response) => {
                println!("{}", response.text().await.unwrap());
            }
            Err(e) => {
                panic!("{}", e.message);
            }
        }
    } else {
        panic!("{:?}", token_result);
    }
}
