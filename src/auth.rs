use std::{env, error::Error};

use bitflags::bitflags;
use hmac::{Hmac, Mac};
use jwt::{Header, Token, Verified};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use uuid::Uuid;

bitflags! {
    /// A bitfield containing the scope of this access token.
    struct Scope: u64 {
        /// This can read the user's email.
        const EMAIL = 1 << 0;
        /// This can read the user's profile.
        const PROFILE = 1 << 1;
        /// This can access the user's post history.
        const POSTS = 1 << 2;
        /// This can create a post on the user's behalf.
        const CREATE_POST = 1 << 3;
        /// This can update a post on the user's behalf.
        const UPDATE_POST = 1 << 4;
        /// This can delete a post on the user's behalf.
        const DELETE_POST = 1 << 5;
    }
}

/// The JWT token claims data.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The scope of the token.
    pub scope: u64,
    /// The time the token was issued.
    pub iat: i64,
    /// The time the token expires.
    pub exp: i64,
    /// The user this token is for.
    pub user: Uuid,
}

/// Fetch the key used for signing JWTs issued by the server.
pub fn get_signing_key() -> Result<Hmac<Sha384>, Box<dyn Error>> {
    let data = base64::decode(env::var("JWT_SIGNING_KEY")?)?;
    Hmac::new_from_slice(&data).map_err(|e| e.into())
}

pub type VerifiedToken = Token<Header, Claims, Verified>;

#[cfg(test)]
mod tests {
    use jwt::{Header, SignWithKey, Token};

    use super::*;

    // don't be stupid and use this in prod.
    static JWT_SIGNING_KEY: &str =
        "kEUJEauCFPV0tcx0zLn+BGAm7j/x9VFN9BPbAVcaSixxSaXsaEl3KSJCu5DK4sBd";

    fn get_test_signing_key() -> Hmac<Sha384> {
        let data = base64::decode(JWT_SIGNING_KEY).unwrap();
        Hmac::new_from_slice(&data).unwrap()
    }

    #[test]
    fn test_scope() {
        let scope = 3;
        let scope = Scope::from_bits(scope).unwrap();
        assert!(scope.contains(Scope::EMAIL));
        assert!(scope.contains(Scope::PROFILE));
    }

    #[test]
    fn test_sign_token() {
        let token = Token::new(
            Header {
                algorithm: jwt::AlgorithmType::Hs384,
                ..Default::default()
            },
            Claims {
                scope: Scope::EMAIL.bits | Scope::PROFILE.bits,
                iat: 0,
                exp: 0,
                user: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
            },
        );

        let key = get_test_signing_key();
        let signed_token = token.sign_with_key(&key).unwrap();
        assert_eq!(signed_token.as_str(), "eyJhbGciOiJIUzM4NCJ9.eyJzY29wZSI6MywiaWF0IjowLCJleHAiOjAsInVzZXIiOiIwMDAwMDAwMC0wMDAwLTAwMDAtMDAwMC0wMDAwMDAwMDAwMDAifQ.EEK0SkRujpTwPGfWitnzmTz043HuuiXTUxUTOMkQo4pXPaEsG-AmUqMToPASCme9");
    }
}
