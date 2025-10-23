# libft-api - 42 Intra API Rust Library

A Rust library that provides typed, asynchronous access to the [42 Intra API](https://api.intra.42.fr/). It wraps common endpoints with strongly typed requests, automatic rate limiting, and reusable session management.

## Features

- **Strong Typing**: All API requests and responses are strongly typed using Rust structs
- **Rate Limiting**: Automatic handling of API rate limits
- **Session Management**: Reusable sessions for making multiple API calls
- **Async Support**: Fully asynchronous API calls using async/await
- **Caching**: Automatic token caching and refresh
- **Error Handling**: Comprehensive error types for different failure scenarios

## Getting Started

### Prerequisites

You need the following two environment variables:

```env
FT_API_CLIENT_UID
FT_API_CLIENT_SECRET
```

These can be obtained by creating an application in your [42 profile settings](https://profile.intra.42.fr/oauth/applications).

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
libft-api = { git = "https://github.com/hdoo42/libft-api" }
tokio = { version = "1.0", features = ["full"] }
```

### Usage

Create a token -> Create a client -> Create a session -> Send API requests!

```rust
use libft_api::prelude::{*, ft_campus_id::GYEONGSAN};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a token from environment variables
    let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
    
    // Create a client
    let client = FtClient::new(FtClientReqwestConnector::new());
    
    // Create a session
    let session = client.open_session(token);
    
    // Send an API request
    let response = session
        .campus_id_locations(
            FtApiCampusIdLocationsRequest::new(FtCampusId::new(GYEONGSAN))
                .with_per_page(100)
        )
        .await?;
    
    for location in response.location {
        println!("{:?} @ {}", location.user.login, location.host);
    }
    
    Ok(())
}
```

## API Implementation Status

### Available Endpoints

The library currently supports the following API endpoints:

#### Campus API
- `GET /campus/:campus_id/journals`
- `GET /campus/:campus_id/locations`
- `GET /campus/:campus_id/users`
- `GET /campus/:campus_id`
- `GET /campus/users`

#### User API
- `GET /users`
- `GET /users/:user_id`
- `GET /users/:user_id/correction_point_historics`
- `POST /users/:user_id/correction_points_add`
- `GET /users/:user_id/locations`
- `GET /users/:user_id/locations_stats`
- `GET /users/:user_id/teams`
- `GET /users/:user_id/cursus_users`
- `GET /users/:user_id/projects_users`

#### Project API
- `GET /projects`
- `GET /projects/:project_id/teams`
- `GET /project_data`

#### Cursus API
- `GET /cursus/:cursus_id/projects`

#### Exam API
- `GET /exams`

#### Group API
- `GET /groups`

#### Project Session API
- `GET /project_sessions/:project_session_id/scale_teams`
- `GET /project_sessions/:project_session_id/teams`

#### Project User API
- `GET /projects_users`

#### Scale Team API
- `GET /scale_teams`

### In Progress

- Additional v3 API coverage
- More endpoint implementations

## Project Structure

- `src/api/` - Endpoint-specific clients for different API domains (campus, user, projects, etc.)
- `src/models/` - Serde-powered representations of API request and response data structures
- `src/auth.rs` - OAuth2 token management and authentication helpers
- `src/common.rs` - Shared utilities, error types, parameters, rate limiters, and pagination
- `src/connector.rs` - HTTP connector implementation using reqwest
- `src/info.rs` - Constants and information about 42 campuses and cursus
- `examples/` - Example implementations demonstrating library usage

## Examples

Check the `examples/` directory for more detailed usage examples:

```bash
cargo run --example scroll
```

This example demonstrates how to use the library to get all users from the Seoul campus and save them to a JSON file.

## Development

### Running Tests

```bash
cargo test
```

Note: Authentication tests require valid API credentials to be set in your environment.

### Building Documentation

```bash
cargo doc --open
```

## Contributing

Contributions are very welcome! Here are some ways you can contribute:

- Report bugs and request features
- Submit pull requests for new API endpoints
- Improve documentation
- Add more comprehensive tests

To contribute a new API endpoint, you'll typically need to:

1. Define the request and response data structures in `src/models/`
2. Create the request/response types in the appropriate module in `src/api/`
3. Implement the API call in the `FtClientSession` extension trait
4. Add tests and examples as appropriate

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

If you need any more help, feel free to open an issue in the repository.
