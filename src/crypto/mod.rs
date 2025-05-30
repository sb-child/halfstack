/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod error;

use std::ops::Deref;

use generic_array::GenericArray;
use jose_jwk::Jwk;
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

    pub fn from_jwk(k: &Jwk) -> Result<Self, error::ParseError> {
        if k.prm.alg != Some(jose_jwa::Algorithm::Signing(jose_jwa::Signing::Es256)) {
            return Err(error::ParseError::InvalidKeyParameters);
        }
        let ec = match &k.key {
            jose_jwk::Key::Ec(ec) => ec,
            _ => return Err(error::ParseError::UnsupportedCurve),
        };
        if ec.crv != jose_jwk::EcCurves::P256 {
            return Err(error::ParseError::UnsupportedCurve);
        }
        let d = &ec.d;
        let Some(d) = d else {
            return Err(error::ParseError::MissingField);
        };
        let da = GenericArray::from_slice(d);
        let sk = SigningKey::from_bytes(da).map_err(|_| error::ParseError::InvalidKeyParameters)?;
        Ok(Self { key: sk })
    }

    pub fn to_jwk(&self) -> Result<Jwk, error::ComposeError> {
        let d = self.key.as_nonzero_scalar();
        let da = d.to_bytes();
        let db = da.deref();
        let p = self.key.verifying_key().to_encoded_point(false);
        let x = p.x().ok_or(error::ComposeError::KeyConversionFailed)?;
        let y = p.y().ok_or(error::ComposeError::KeyConversionFailed)?;
        let xb = x.deref();
        let yb = y.deref();
        let curve = jose_jwk::Ec {
            crv: jose_jwk::EcCurves::P256,
            d: Some(db.to_vec().into()),
            x: xb.to_vec().into(),
            y: yb.to_vec().into(),
        };
        let key = jose_jwk::Key::Ec(curve);
        let mut prm = jose_jwk::Parameters::default();
        prm.alg = Some(jose_jwa::Algorithm::Signing(jose_jwa::Signing::Es256));
        let jwk = Jwk { key, prm };
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
    pub fn from_jwk(k: &Jwk) -> Result<Self, error::ParseError> {
        if k.prm.alg != Some(jose_jwa::Algorithm::Signing(jose_jwa::Signing::Es256)) {
            return Err(error::ParseError::InvalidKeyParameters);
        }
        let ec = match &k.key {
            jose_jwk::Key::Ec(ec) => ec,
            _ => return Err(error::ParseError::UnsupportedCurve),
        };
        if ec.crv != jose_jwk::EcCurves::P256 {
            return Err(error::ParseError::UnsupportedCurve);
        }
        let x = &ec.x;
        let y = &ec.y;
        let xa = GenericArray::from_slice(x);
        let ya = GenericArray::from_slice(y);
        let p = EncodedPoint::from_affine_coordinates(xa, ya, false);
        let pk = VerifyingKey::from_encoded_point(&p)
            .map_err(|_| error::ParseError::InvalidKeyParameters)?;
        Ok(Self { key: pk })
    }

    pub fn to_jwk(&self) -> Result<Jwk, error::ComposeError> {
        let p = self.key.to_encoded_point(false);
        let x = p.x().ok_or(error::ComposeError::KeyConversionFailed)?;
        let y = p.y().ok_or(error::ComposeError::KeyConversionFailed)?;
        let xb = x.deref();
        let yb = y.deref();
        let curve = jose_jwk::Ec {
            crv: jose_jwk::EcCurves::P256,
            d: None,
            x: xb.to_vec().into(),
            y: yb.to_vec().into(),
        };
        let key = jose_jwk::Key::Ec(curve);
        let mut prm = jose_jwk::Parameters::default();
        prm.alg = Some(jose_jwa::Algorithm::Signing(jose_jwa::Signing::Es256));
        let jwk = Jwk { key, prm };
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
