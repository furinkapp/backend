use hmac::Hmac;
use jwt::{Header, Token, Verified, VerifyWithKey};
use sha2::Sha384;
use warp::{Filter, Rejection};

use crate::{
    auth::{get_signing_key, Claims},
    errors::ServerError,
};

/// Available environments.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Env {
    /// The `development` environment.
    Dev,
    /// The `production` environment.
    Prod,
}

/// A filter that only matches if the environment is `Dev`.
pub fn is_dev() -> impl Filter {
    is_env(Env::Dev)
}

/// A filter that only matches if the environment is `Prod`.
pub fn is_prod() -> impl Filter {
    is_env(Env::Prod)
}

fn is_env(target: Env) -> impl Filter {
    warp::any().and_then(move || async move {
        #[cfg(debug_assertions)]
        let env = Env::Dev;
        #[cfg(not(debug_assertions))]
        let env = Env::Prod;

        if target == env {
            Ok(())
        } else {
            Err(warp::reject::not_found())
        }
    })
}

/// Extracts and validates a token from the request.
pub fn access_token(
) -> impl Filter<Extract = (Token<Header, Claims, Verified>,), Error = Rejection> + Copy {
    warp::any()
        .and(warp::header("Authorization"))
        .and_then(move |token: String| async move {
            if !token.starts_with("Bearer ") {
                return Err(warp::reject::custom(ServerError::InvalidHeader(token)));
            }
            let token = &token[7..];
            // Fetch the signing key from the environment.
            let key: Hmac<Sha384> = match get_signing_key() {
                Ok(key) => key,
                Err(_) => return Err(warp::reject::custom(ServerError::MissingSigningKey)),
            };
            // Verify the token.
            let token: Token<Header, Claims, Verified> =
                VerifyWithKey::verify_with_key(token, &key)
                    .map_err(|_| warp::reject::custom(ServerError::InvalidSignature))?;
            // Return the token.
            Ok(token)
        })
}
