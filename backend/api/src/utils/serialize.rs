use secp256k1::PublicKey;
use serde::{de, Deserialize, Deserializer, Serializer};
use uuid::Uuid;

pub fn serialize_pubkey<S>(v: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let key_hex = hex::encode(v.serialize()); // Convert PublicKey to hex string
    serializer.serialize_str(&key_hex)
}

impl<'de> de::Deserialize<'de> for PublicKey {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(Path(s))
    }
}
