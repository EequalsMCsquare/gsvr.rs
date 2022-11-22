use gconf::ConfigJwt;
pub use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use jsonwebtoken::{Header, Validation};
use serde::{de::DeserializeOwned, Serialize};
use std::{io::Read, marker::PhantomData, str::FromStr, sync::Arc};

#[derive(Clone)]
pub struct Jwt<T> {
    inner: Arc<Inner>,
    _t: PhantomData<T>,
}

struct Inner {
    enc_key: EncodingKey,
    dec_key: DecodingKey,
    validation: Validation,
    header: Header,
}

impl<T> Jwt<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn new(algorithm: Algorithm, encoding_key: EncodingKey, decoding_key: DecodingKey) -> Self {
        Self {
            inner: Arc::new(Inner {
                enc_key: encoding_key,
                dec_key: decoding_key,
                validation: Validation::default(),
                header: Header::new(algorithm),
            }),
            _t: PhantomData,
        }
    }

    pub fn from_config(cfg: &ConfigJwt) -> Result<Self, Box<dyn std::error::Error>> {
        // load file
        let enckey = if cfg.encode_key.starts_with("file:") {
            let mut buf = String::new();
            std::fs::File::open(cfg.encode_key.replace("file:", ""))?.read_to_string(&mut buf)?;
            buf
        } else {
            cfg.encode_key.clone()
        };
        let deckey = if cfg.decode_key.starts_with("file:") {
            let mut buf = String::new();
            std::fs::File::open(cfg.decode_key.replace("file:", ""))?.read_to_string(&mut buf)?;
            buf
        } else {
            cfg.decode_key.clone()
        };
        let alg = Algorithm::from_str(&cfg.algorithm)?;
        match alg {
            // HMAC family
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => Ok(Self {
                inner: Arc::new(Inner {
                    enc_key: EncodingKey::from_secret(enckey.as_bytes()),
                    dec_key: DecodingKey::from_secret(deckey.as_bytes()),
                    validation: Validation::new(alg),
                    header: Header::new(alg),
                }),
                _t: PhantomData,
            }),
            // RSA family
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => Ok(Self {
                inner: Arc::new(Inner {
                    enc_key: EncodingKey::from_rsa_pem(enckey.as_bytes())?,
                    dec_key: DecodingKey::from_rsa_pem(deckey.as_bytes())?,
                    validation: Validation::new(alg),
                    header: Header::new(alg),
                }),
                _t: PhantomData,
            }),
            // EC family
            Algorithm::ES256 | Algorithm::ES384 => Ok(Self {
                inner: Arc::new(Inner {
                    enc_key: EncodingKey::from_ec_pem(enckey.as_bytes())?,
                    dec_key: DecodingKey::from_ec_pem(deckey.as_bytes())?,
                    validation: Validation::new(alg),
                    header: Header::new(alg),
                }),
                _t: PhantomData,
            }),
            // ED family
            Algorithm::EdDSA => Ok(Self {
                inner: Arc::new(Inner {
                    enc_key: EncodingKey::from_ed_pem(enckey.as_bytes())?,
                    dec_key: DecodingKey::from_ed_pem(deckey.as_bytes())?,
                    validation: Validation::new(alg),
                    header: Header::new(alg),
                }),
                _t: PhantomData,
            }),
        }
    }

    pub fn encode(&self, claim: T) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&self.inner.header, &claim, &self.inner.enc_key)
    }

    pub fn decode(&self, token: &str) -> Result<T, jsonwebtoken::errors::Error> {
        match jsonwebtoken::decode(token, &self.inner.dec_key, &self.inner.validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(err),
        }
    }
}
