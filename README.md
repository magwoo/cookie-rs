![cercis-preview](./.github/assets/preview.png)

# Cookie Library

`cookie-rs` is a flexible library for working with HTTP cookies. It allows you to create, parse, and manage cookies.

## Features

- Create cookies with various attributes (e.g., `Domain`, `Path`, `Secure`, `HttpOnly`).
- Parse cookies from HTTP headers.
- Manage cookies using `CookieJar`, which tracks additions and removals.
- Support for `SameSite` attribute.
- Errors are handled gracefully through `ParseError`.

## Quick Start

To use this library, add it to your dependencies:

```diff
[dependencies]
...
+ cookie-rs = "0.2.0"
```

### Create a Cookie

```rust
use cookie_rs::Cookie;
let cookie = Cookie::builder("session", "abc123")
    .domain("example.com")
    .path("/")
    .secure(true)
    .http_only(true)
    .same_site(cookie_rs::SameSite::Lax)
    .build();
println!("{}", cookie.to_string());
```

Output:

```
session=abc123; Domain=example.com; Path=/; Secure; HttpOnly; SameSite=Lax
```

### Parse a Cookie

```rust
use cookie_rs::Cookie;
let cookie_str = "session=abc123; Secure; HttpOnly";
let cookie = Cookie::parse(cookie_str).expect("Failed to parse cookie");
assert_eq!(cookie.name(), "session");
assert_eq!(cookie.value(), "abc123");
assert_eq!(cookie.secure(), Some(true));
assert_eq!(cookie.http_only(), Some(true));
```

### Manage Cookies with `CookieJar`

```rust
use cookie_rs::{Cookie, CookieJar};
let mut jar = CookieJar::default();
// Add a cookie
let cookie = Cookie::new("user", "john");
jar.add(cookie);
// Retrieve a cookie
if let Some(cookie) = jar.get("user") {
    println!("Found cookie: {}={}.", cookie.name(), cookie.value());
}
// Remove a cookie
jar.remove("user");
assert!(jar.get("user").is_none());
```

### Parse Multiple Cookies from a Header

```rust
use cookie_rs::CookieJar;
let cookie_header = "name1=value1; name2=value2";
let jar = CookieJar::parse(cookie_header).expect("Failed to parse cookies");
assert!(jar.get("name1").is_some());
assert!(jar.get("name2").is_some());
```

### Convert Cookies to HTTP Header Values

```rust
use cookie_rs::{Cookie, CookieJar};
let mut jar = CookieJar::default();
jar.add(Cookie::new("name1", "value1"));
jar.add(Cookie::new("name2", "value2"));
let headers = jar.as_header_values();
for header in headers {
    println!("Set-Cookie: {}", header);
}
```

Output:

```
Set-Cookie: name1=value1
Set-Cookie: name2=value2
```
