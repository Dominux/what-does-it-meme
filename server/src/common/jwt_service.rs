use itertools::Itertools;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

use super::errors::{MemeError, MemeResult};

pub struct JWTService<'a> {
    secret: &'a [u8],
}

const ALGORITHM: Algorithm = Algorithm::EdDSA;
const TOKEN_TYPE: &str = "Bearer";

impl<'a> JWTService<'a> {
    pub fn new(secret: &'a str) -> Self {
        Self {
            secret: secret.as_ref(),
        }
    }

    #[inline]
    pub fn encode<'b, T: Serialize>(&self, claims: &'b T) -> MemeResult<String> {
        let token = encode(
            &Header::new(ALGORITHM),
            claims,
            &EncodingKey::from_secret(self.secret),
        )
        .map_err(|e| MemeError::JWTError)?;

        Ok(format!("{} {}", TOKEN_TYPE, token))
    }

    pub fn decode<'b, T: DeserializeOwned>(&self, token: &'b str) -> MemeResult<T> {
        // Firstable splitting token onto it's type and token itself, validating type
        let token = self.parse_token(token.to_string())?;

        let tokendata = decode(
            token.as_str(),
            &DecodingKey::from_secret(self.secret),
            &Validation::new(ALGORITHM),
        );
        match tokendata {
            Ok(tokendata) => Ok(tokendata.claims),
            Err(_) => Err(MemeError::InvalidToken),
        }
    }

    fn parse_token(&self, token: String) -> MemeResult<String> {
        let (token_type, token_data): (&str, &str) = token
            .split(" ")
            .collect_tuple()
            .ok_or(MemeError::InvalidToken)?;

        if token_type == TOKEN_TYPE {
            Ok(token_data.to_string())
        } else {
            Err(MemeError::InvalidToken)
        }
    }
}
