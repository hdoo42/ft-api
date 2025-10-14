# API Structure Relationships

## Mermaid Diagram

```mermaid
classDiagram
    class FtClientHttpConnector {
        <<trait>>
        +http_get_uri(full_uri, token, ratelimiter)$
        +http_post_uri(full_uri, token, request_body)$
        +http_patch_uri(full_uri, token, request_body)$
        +http_delete_uri(full_uri, token, request_body)$
        +create_method_uri_path(method_relative_uri)$
    }
    
    class FtClientReqwestConnector {
        -reqwest_connector: Client
        -ft_api_url: String
        +new()$
        +with_connector(connector)$
        +with_ft_api_url(ft_api_url)$
        -send_http_request(reqwest, url, meta)$
    }
    
    class FtClientHttpApiUri {
        +FT_API_URI_STR: String
        +create_method_uri_path(method_relative_uri)$
        +create_url_with_params(base_url, params)$
    }
    
    class FtClient~FCHC~ {
        +http_api: FtClientHttpApi~FCHC~
        +meta: HeaderMetaData
        +new(http_connector)$
        +with_ratelimits(http_connector, secondly, hourly)$
        +open_session(token)$
    }
    
    class FtClientHttpApi~FCHC~ {
        +connector: Arc~FCHC~
        +new(http_connector)$
    }
    
    class FtClientSession~'a, FCHC~ {
        +http_session_api: FtClientHttpSessionApi~'a, FCHC~
    }
    
    class FtClientHttpSessionApi~'a, FCHC~ {
        -token: FtApiToken
        +client: &FtClient~FCHC~
        +http_get_uri(full_uri)$
        +http_get(method_relative_uri, params)$
        +http_post(method_relative_uri, request)$
        +http_post_uri(full_uri, request)$
        +http_delete(method_relative_uri, request)$
        +http_delete_uri(full_uri, request)$
    }
    
    class FtApiToken {
        -access_token: String
        -token_type: AccessTokenType
        -expires_in: i64
        -scope: String
        -created_at: i64
        -secret_valid_until: i64
        +get_token_value()$
    }
    
    class HeaderMetaData {
        -ratelimiter: RateLimiter
        +new(ratelimiter)$
    }
    
    class ClientResult~T~ {
        <<type alias>>
    }

    %% Relationships
    FtClientHttpConnector <|-- FtClientReqwestConnector : implements
    FtClientReqwestConnector --> FtClientHttpApiUri : uses
    FtClientReqwestConnector --> FtApiToken : uses
    FtClientReqwestConnector --> HeaderMetaData : uses
    
    FtClientHttpApi <--> FtClientHttpConnector : generic bound
    FtClientHttpApi --> FtClientReqwestConnector : stores in Arc
    
    FtClient --> FtClientHttpApi : contains
    FtClient --> HeaderMetaData : contains
    FtClient --> FtClientReqwestConnector : generic bound
    
    FtClientSession --> FtClientHttpSessionApi : contains
    FtClientSession --> FtClient : contains reference to
    FtClientSession --> FtApiToken : through http_session_api
    
    FtClientHttpSessionApi --> FtClient : contains reference
    FtClientHttpSessionApi --> FtApiToken : contains
    FtClientHttpSessionApi --> FtClientHttpApi : uses connector
    FtClientHttpSessionApi --> FtClientHttpConnector : through connector
    
    FtClientReqwestConnector ..> ClientResult : returns
    FtClientSession ..> ClientResult : returns
    FtClientHttpSessionApi ..> ClientResult : returns
    
    %% Inheritance relationship
    FtClientReqwestConnector --|> FtClientHttpConnector
    FtClientHttpApi --|> FtClientHttpConnector
    FtClientHttpSessionApi --|> FtClientHttpConnector
```

## Explanation of Relationships

### Core Components

1. **FtClientHttpConnector (Trait)**: Defines the interface for HTTP connectors that can communicate with the 42 API. It has methods for GET, POST, PATCH, DELETE requests and URI creation.

2. **FtClientReqwestConnector (Implementation)**: A concrete implementation of FtClientHttpConnector using the reqwest HTTP client library. It handles the actual network communication with the 42 Intra API.

3. **FtClient<FCHC>**: The main client structure that serves as the entry point for API interactions. It contains:
   - `http_api`: An instance of FtClientHttpApi that manages the connector
   - `meta`: HeaderMetaData containing rate limiting information

4. **FtClientSession<'a, FCHC>**: An authenticated session that holds a valid token and allows making authenticated API calls.

### Data Flow

1. User creates an `FtClient` with a connector (typically `FtClientReqwestConnector`)
2. User opens a session with a valid `FtApiToken` using `client.open_session(token)`
3. User makes API calls through the session (e.g., `session.users()`)
4. The session uses its internal `FtClientHttpSessionApi` to make requests
5. The session API calls the underlying connector to perform HTTP operations
6. The connector handles authentication, rate limiting, and communication with the API

### Thread Safety and Concurrency

- `FtClientHttpApi` stores the connector in an `Arc<FCHC>` to allow sharing across threads
- All connector implementations must implement `Send` and `Sync` for thread safety
- The generic `FCHC` parameter allows for different connector implementations while maintaining type safety

This architecture allows for flexible connector implementations while providing a consistent API interface for users of the library.
