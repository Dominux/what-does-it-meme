use std::str::FromStr;

use actix_web::http::header;

const AUTH_HEADER: &str = "Authotization";

pub struct AuthorizationHeader {
    pub token: String,
}

impl header::Header for AuthorizationHeader {
    fn name() -> header::HeaderName {
        header::HeaderName::from_str(AUTH_HEADER).expect("Fatal Error: invalid header name")
    }

    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
        msg.headers()
            .get_all(Self::name())
            .find_map(|header| header.to_str().ok())
            .ok_or(actix_web::error::ParseError::Header)
            .map(|header| AuthorizationHeader {
                token: header.to_string(),
            })
    }
}

impl header::TryIntoHeaderValue for AuthorizationHeader {
    type Error = header::InvalidHeaderValue;

    #[inline]
    fn try_into_value(self) -> Result<header::HeaderValue, Self::Error> {
        header::HeaderValue::from_str(AUTH_HEADER)
    }
}
