use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

const KEY: &[u8] = b"01234567890123456789012345678901";
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    #[serde(rename = "privateAge")]
    age: u32,
    date_of_birth: DateTime<Utc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    skills: Vec<String>,
    state: WorkState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,
    // #[serde(serialize_with = "serialize_encrypt", deserialize_with = "deserialize_decrypt")]
    #[serde_as(as = "DisplayFromStr")]
    sensitive: SensitiveData,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    url: Vec<http::Uri>,
}
#[derive(Debug, PartialEq)]
struct SensitiveData(String);
#[derive(Debug, Deserialize, Serialize, PartialEq)]
//#[serde(rename_all = "snake_case")]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
enum WorkState {
    Working(String),
    OnLeave(DateTime<Utc>),
    Terminated,
}

fn main() -> anyhow::Result<()> {
    // let state = WorkState::Working("Rust".to_string());
    let state1 = WorkState::OnLeave(Utc::now());
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        date_of_birth: Utc::now(),
        skills: vec!["Rust".to_string(), "Python".to_string()],
        state: state1,
        data: vec![1, 2, 3, 4],
        sensitive: SensitiveData::new("secret"),
        url: vec!["http://example.com/a".parse()?],
    };
    let host = user.url[0].host();
    println!("{:?}", host);
    let json = serde_json::to_string(&user)?;
    println!("{}", json);
    let user1: User = serde_json::from_str(&json)?;
    println!("{:?}", user1);
    // let user = User {
    //     name: "Alice".to_string(),
    //     age: 30,
    //     dob: Utc::now(),
    //     skills: vec!["Rust".to_string(), "javascript".to_string()],
    // };

    Ok(())
}

fn b64_encode<S>(data: &Vec<u8>, serialize: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encode = URL_SAFE_NO_PAD.encode(data);
    serialize.serialize_str(&encode)
}
fn b64_decode<'de, S>(deserializer: S) -> Result<Vec<u8>, S::Error>
where
    S: serde::Deserializer<'de>,
{
    println!("555");
    let encoded = String::deserialize(deserializer)?;
    let decode = URL_SAFE_NO_PAD
        .decode(encoded.as_bytes())
        .map_err(serde::de::Error::custom)?;
    Ok(decode)
}
#[allow(dead_code)]
fn serialize_encrypt<S>(data: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encrypted = encrypt(data.as_bytes()).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&encrypted)
}
#[allow(dead_code)]
fn deserialize_decrypt<'de, S>(deserializer: S) -> Result<String, S::Error>
where
    S: serde::Deserializer<'de>,
{
    let encrypted = String::deserialize(deserializer)?;
    let decrypted = decrpt(&encrypted).map_err(serde::de::Error::custom)?;
    let decrypted = String::from_utf8(decrypted).map_err(serde::de::Error::custom)?;
    Ok(decrypted)
}

fn encrypt(data: &[u8]) -> anyhow::Result<String> {
    println!("{}", KEY.len());
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data).unwrap();
    let nonce_cryertext: Vec<_> = nonce.iter().copied().chain(ciphertext).collect();

    Ok(URL_SAFE_NO_PAD.encode(nonce_cryertext))
}
fn decrpt(decode: &str) -> anyhow::Result<Vec<u8>> {
    let decoded = URL_SAFE_NO_PAD.decode(decode.as_bytes())?;
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = decoded[..12].into();
    let decrypted = cipher.decrypt(nonce, &decoded[12..]).unwrap();
    Ok(decrypted)
}
impl fmt::Display for SensitiveData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let encrypted = encrypt(self.0.as_bytes()).unwrap();
        write!(f, "{}", encrypted)
    }
}
impl FromStr for SensitiveData {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let decrpted = decrpt(s)?;
        let decrpted = String::from_utf8(decrpted)?;
        Ok(Self(decrpted))
    }
}
impl SensitiveData {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}
