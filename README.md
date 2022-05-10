# Firebase Realtime Database Wrapper
A basic abstraction over the Firebase Web API.

To begin, create your OAuth token by passing the path to your service account token file.

```rs
use firebase_realtime_database::*;

let token = get_oauth_token("path/to/token/file.json").await?;
```

With that token, we create a database with the realtime database name and that token
```rs
let database = create_database("database-name-default-rtdb", token.as_str());
```

Finally, the database exposes four methods to interact with the database
```rs
let get_result = database.get("users/tom").await?;

let put_result = database
    .put(
        "users/joe",
        &HashMap::from([("first_name", "Joe"), ("last_name", "Mama")]),
    )
    .await?;

let update_result = database
    .update("users/joe", &HashMap::from([("last_name", "Ma'ma")]))
    .await?;

let delete_result = database.delete("users/joe2").await?;
```

Where each method takes a path. Put, Update and Delete take in Serializable data. All methods return a result of reqwest::Response and FirebaseError. FirebaseError contains just a message field that contains the error message.