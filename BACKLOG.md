# Project Backlog

## Token Management

### High Priority
- [ ] **Token refresh race condition**: Under heavy concurrent load, multiple threads could bypass the double-check pattern and attempt simultaneous token refreshes. Implement a semaphore or dedicated token refresh task to serialize refresh attempts.
- [ ] **Add timeout to token requests**: Token refresh requests lack timeouts, which could cause indefinite blocking. Add configurable timeout for authentication calls.
- [ ] **Implement token refresh retry logic**: Failed token refreshes (e.g., temporary network issues) propagate immediately. Add exponential backoff retry strategy.

### Medium Priority
- [ ] **Make token expiration buffer configurable**: The 60-second buffer is hardcoded. Allow configuration for different use cases.

## Error Handling

### High Priority
- [ ] **Improve error parsing robustness**: `handle_error()` silently swallows deserialization errors with `unwrap_or()`. Log or return more information about malformed API responses.
- [ ] **Add error source chains**: Implement error source information to improve debugging capability.

### Medium Priority
- [ ] **Auto-retry on rate limiting**: Detect 429 responses and implement automatic retry with exponential backoff instead of requiring users to implement their own logic.
- [ ] **Improve error type coverage**: Ensure all Auth0 error response types are properly captured and reported.

## Request/Response Handling

### High Priority
- [ ] **Implement pagination helpers**: Add iterator or async stream support for paginated results so users don't need to manually handle pagination loops.
- [ ] **Use PaginatedResponse in list endpoints**: Currently defined but unused. Refactor list endpoints to return proper pagination metadata.

### Medium Priority
- [ ] **Preserve response data for delete operations**: Return metadata from delete responses instead of discarding with `()`.
- [ ] **Inspect response headers**: Capture `X-RateLimit-*`, `X-Total-Count`, and other critical headers for better visibility into API behavior.
- [ ] **Fix potential URL encoding issues**: Audit for double-encoding edge cases between `urlencoding::encode()` and `url.query_pairs_mut()` approaches.

## Configuration & Validation

### High Priority
- [ ] **Validate domain format**: Check domain format during builder configuration, not just at URL parse time.
- [ ] **Normalize URLs to prevent double slashes**: Fix audience auto-generation to handle trailing slashes correctly.
- [ ] **Validate credentials**: Check that client_id and client_secret are non-empty strings in the builder.

### Medium Priority
- [ ] **Add HTTP client customization**: Allow users to configure reqwest client timeouts, proxy, retry policies, and other behavior.
- [ ] **Support custom headers**: Enable users to add custom headers (correlation IDs, API versioning hints) to all requests.

## API Coverage

### High Priority
- [ ] **Fix Cargo.toml edition**: Change `edition = "2024"` to `"2021"`.

### Medium Priority
- [ ] **Add logging/debugging support**: Implement request/response logging for easier debugging without external interceptors.
- [ ] **Add connection-specific endpoints**: Implement `/api/v2/connections/{id}/users` and other connection-scoped operations.
- [ ] **Support bulk operations**: Expose `/jobs/users-imports` and related bulk operation endpoints.
- [ ] **Add client credentials rotation**: Allow ManagementClient to rotate its own authentication credentials after initialization.

### Low Priority
- [ ] **Add v1 API support**: Support legacy Management API v1 endpoints for backward compatibility.

## Type Safety & Validation

### Medium Priority
- [ ] **Validate empty update requests**: Prevent construction of update requests with all None values that produce empty JSON objects.
- [ ] **Validate connection names**: Check that connection values exist in Auth0 during request construction if possible.

### Low Priority
- [ ] **Improve metadata typing**: Replace `serde_json::Map` with strongly-typed metadata structures or provide validation helpers.

## Concurrency & Performance

### Medium Priority
- [ ] **Optimize token lock contention**: Reduce RwLock hold time during token refresh by separating the request from the state update.
- [ ] **Add optional request throttling**: Implement configurable request queue/rate limiting to prevent overwhelming the API or exhausting rate limits.

## Feature Flags & Dependencies

### Low Priority
- [ ] **Validate feature flag dependencies**: Ensure feature flags properly declare cross-module dependencies to prevent invalid configurations.
