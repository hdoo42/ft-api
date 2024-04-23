# ft-api
I've made the 42 API usable with Rust.

You need the following two environment variables:
```env
FT_API_CLIENT_UID
FT_API_CLIENT_SECRET
```

## Example

Create a token -> Create a client -> Create a session (simple wrapper) -> Send API requests!
```rust 
    //build a token
    let res = FtApiToken::build_from_env().await;

    if let Ok(token) = res {
        println!("token ok");
        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session.campus_gs_locations().await?;
      // res will contain all the locations for campus gs(Gyeongsan)
    }
```

## Plans
There are two major components that need to be implemented:

1. API Request Implementation
This involves setting up the functions and methods necessary to send requests to the 42 API.

2. Data Structures for API Responses
This entails defining Rust structs that will map to the JSON data returned by the 42 API. These structures will be used to deserialize the API responses into Rust object.

## What is done?

- oauth

### v2
- campus/:campus_id:/locations


### v3


## Contribute?

Contributions are very welcome. 

Let me know if you need any more help!
