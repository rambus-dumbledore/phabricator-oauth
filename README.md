# Phabricator-OAuth

**Library for OAuth2 authentication on [Phabricator](https://www.phacility.com/phabricator/)**

---

Example
```rust
use phabricator_oauth::*;
...
let phid = String::from("PHABRICATOR_ID");
let secret = String::from("PHABRICATOR_SECRET");
let redirect_url = String::from("https://yourservice.com/auth"); // Exactly like in oauth settings on phabricator
let phabricator_url = String::from("https://phabricator.yourdomain.com");

let client = PhabOAuthClient::new(phid, secret, redirect_url, phabricator_url).unwrap();
...
// Getting URL for authentication on the phabricator
let redirect_url = client.get_auth_url().unwrap();
...
// Getting OAuth token. Code will be in GET parameters in your /auth handler
let token = client.get_token(code.to_string()).unwrap();
let access_token = token.access_token();
...
// Getting current user info
let user = client.get_user(access_token).unwrap();
```
