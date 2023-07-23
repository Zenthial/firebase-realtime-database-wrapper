# Firebase Realtime Database Wrapper
A basic abstraction over the Firebase Web API.

To begin, create a Database struct by passing the path to your service account token file, and your project id"

```rs
use firebase_realtime_database::Database;

let database = Database::from_path("project-id", "path/to/token/file.json").await?;
```

The database exposes four methods to interact with the database
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