use ff::PrimeField;
use group::GroupEncoding;
use serde::{de::Error as DeserializeError, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    BN256Affine, BN256Compressed, Fq, Fr, Grumpkin, GrumpkinAffine, GrumpkinCompressed, BN256,
};
use group::Curve;

/// Serializes bytes to human readable or compact representation.
///
/// Depending on whether the serializer is a human readable one or not, the bytes are either
/// encoded as a hex string or a list of bytes.
fn serialize_bytes<S: Serializer>(bytes: [u8; 32], s: S) -> Result<S::Ok, S::Error> {
    if s.is_human_readable() {
        hex::serde::serialize(bytes, s)
    } else {
        bytes.serialize(s)
    }
}

/// Deserialize bytes from human readable or compact representation.
///
/// Depending on whether the deserializer is a human readable one or not, the bytes are either
/// decoded from a hex string or a list of bytes.
fn deserialize_bytes<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
    if d.is_human_readable() {
        hex::serde::deserialize(d)
    } else {
        <[u8; 32]>::deserialize(d)
    }
}

impl Serialize for Fq {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_repr(), s)
    }
}

impl<'de> Deserialize<'de> for Fq {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        match Fq::from_repr(bytes).into() {
            Some(fq) => Ok(fq),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Pallas field element",
            )),
        }
    }
}

impl Serialize for Fr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_repr(), s)
    }
}

impl<'de> Deserialize<'de> for Fr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        match Fr::from_repr(bytes).into() {
            Some(fr) => Ok(fr),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Bn254 field element",
            )),
        }
    }
}

impl Serialize for GrumpkinAffine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_bytes().0, s)
    }
}

impl<'de> Deserialize<'de> for GrumpkinAffine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        let com = GrumpkinCompressed(bytes);
        match GrumpkinAffine::from_bytes(&com).into() {
            Some(g1_affine) => Ok(g1_affine),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Grumpkin curve point",
            )),
        }
    }
}

impl Serialize for BN256Affine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_bytes(self.to_bytes().0, s)
    }
}

impl<'de> Deserialize<'de> for BN256Affine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = deserialize_bytes(d)?;
        let com = BN256Compressed(bytes);
        match BN256Affine::from_bytes(&com).into() {
            Some(g1_affine) => Ok(g1_affine),
            None => Err(D::Error::custom(
                "deserialized bytes don't encode a Grumpkin curve point",
            )),
        }
    }
}

impl Serialize for Grumpkin {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        GrumpkinAffine::serialize(&self.to_affine(), s)
    }
}

impl<'de> Deserialize<'de> for Grumpkin {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(GrumpkinAffine::deserialize(d)?))
    }
}

impl Serialize for BN256 {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        BN256Affine::serialize(&self.to_affine(), s)
    }
}

impl<'de> Deserialize<'de> for BN256 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(BN256Affine::deserialize(d)?))
    }
}

impl Serialize for GrumpkinCompressed {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for GrumpkinCompressed {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self(<[u8; 32]>::deserialize(d)?))
    }
}

impl Serialize for BN256Compressed {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for BN256Compressed {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self(<[u8; 32]>::deserialize(d)?))
    }
}
