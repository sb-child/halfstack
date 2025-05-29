/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod error;

use std::ops::Deref;

use jsonwebkey::{ByteArray, Curve, JsonWebKey};
use p256::{
    EncodedPoint,
    ecdsa::{SigningKey, VerifyingKey},
};

#[derive(PartialEq, Eq, Debug)]
pub struct ES256 {
    key: SigningKey,
}

impl ES256 {
    pub fn new() -> Self {
        let mut rng = rand_old::rngs::OsRng;
        let prikey = SigningKey::random(&mut rng);
        Self { key: prikey }
    }

    pub fn from_jwk(k: &JsonWebKey) -> Result<Self, error::ParseError> {
        let key = k.key.clone();
        let curve = match *key {
            jsonwebkey::Key::EC { curve } => curve,
            _ => {
                return Err(error::ParseError::InvalidJwkFormat);
            }
        };
        let d = match curve {
            Curve::P256 { d, x: _, y: _ } => d,
            _ => {
                return Err(error::ParseError::UnsupportedCurve);
            }
        };
        let d = d.ok_or(error::ParseError::MissingField)?;
        let db = d.deref();
        let sk = SigningKey::from_bytes(db.into())
            .map_err(|_| error::ParseError::InvalidKeyParameters)?;
        Ok(Self { key: sk })
    }

    pub fn to_jwk(&self) -> Result<JsonWebKey, error::ComposeError> {
        let d = self.key.as_nonzero_scalar();
        let da = d.to_bytes();
        let db = da.deref();
        let p = self.key.verifying_key().to_encoded_point(false);
        let x = p.x().ok_or(error::ComposeError::KeyConversionFailed)?;
        let y = p.y().ok_or(error::ComposeError::KeyConversionFailed)?;
        let xb = x.deref();
        let yb = y.deref();
        let curve = Curve::P256 {
            d: Some(ByteArray::from_slice(db)),
            x: ByteArray::from_slice(xb),
            y: ByteArray::from_slice(yb),
        };
        let key = jsonwebkey::Key::EC { curve };
        let mut jwk = JsonWebKey::new(key);
        jwk.set_algorithm(jsonwebkey::Algorithm::ES256)
            .map_err(|_| error::ComposeError::InvalidKeyParameters)?;
        Ok(jwk)
    }

    pub fn pubkey(&self) -> ES256Pub {
        ES256Pub {
            key: *self.key.verifying_key(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ES256Pub {
    key: VerifyingKey,
}

impl ES256Pub {
    pub fn from_jwk(k: &JsonWebKey) -> Result<Self, error::ParseError> {
        let key = k.key.clone();
        let curve = match *key {
            jsonwebkey::Key::EC { curve } => curve,
            _ => return Err(error::ParseError::InvalidJwkFormat),
        };
        let (x, y) = match curve {
            Curve::P256 { d: _, x, y } => (x, y),
            _ => return Err(error::ParseError::UnsupportedCurve),
        };
        let xb = x.deref();
        let yb = y.deref();
        let p = EncodedPoint::from_affine_coordinates(xb.into(), yb.into(), false);
        let pk = VerifyingKey::from_encoded_point(&p)
            .map_err(|_| error::ParseError::InvalidKeyParameters)?;
        Ok(Self { key: pk })
    }

    pub fn to_jwk(&self) -> Result<JsonWebKey, error::ComposeError> {
        let p = self.key.to_encoded_point(false);
        let x = p.x().ok_or(error::ComposeError::KeyConversionFailed)?;
        let y = p.y().ok_or(error::ComposeError::KeyConversionFailed)?;
        let xb = x.deref();
        let yb = y.deref();
        let curve = Curve::P256 {
            d: None,
            x: ByteArray::from_slice(xb),
            y: ByteArray::from_slice(yb),
        };
        let key = jsonwebkey::Key::EC { curve };
        let mut jwk = JsonWebKey::new(key);
        jwk.set_algorithm(jsonwebkey::Algorithm::ES256)
            .map_err(|_| error::ComposeError::InvalidKeyParameters)?;
        Ok(jwk)
    }
}

mod test {
    #[test]
    fn test_jwk() -> Result<(), Box<dyn std::error::Error>> {
        use super::*;
        let es256_sk = ES256::new();
        let es256_sk_jwk = es256_sk.to_jwk()?;
        // dbg!(&es256_sk_jwk);
        let recovered_es256_sk = ES256::from_jwk(&es256_sk_jwk)?;
        // let recovered_es256_sk_jwk = recovered_es256_sk.to_jwk()?;
        // dbg!(&recovered_es256_sk_jwk);
        assert_eq!(&es256_sk, &recovered_es256_sk);
        let es256_pk = es256_sk.pubkey();
        let es256_pk_jwk = es256_pk.to_jwk()?;
        // dbg!(&es256_pk_jwk);
        let recovered_es256_pk = ES256Pub::from_jwk(&es256_pk_jwk)?;
        // let recovered_es256_pk_jwk = recovered_es256_pk.to_jwk()?;
        // dbg!(&recovered_es256_pk_jwk);
        assert_eq!(&es256_pk, &recovered_es256_pk);
        Ok(())
    }
}
