use chrono::{DateTime, Utc};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    dob: DateTime<Utc>,
    skills: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum MyState {
    Init(String),
    Running(Vec<String>),
    Done(u32),
}
fn main() -> anyhow::Result<()> {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        dob: Utc::now(),
        skills: vec!["Rust".to_string(), "javascript".to_string()],
    };
    let json = serde_json::to_string(&user)?;
    println!("{json:?}");

    let user1: User = serde_json::from_str(&json)?;
    println!("{:?}", user1);
    assert_eq!(user1, user);

    let state = MyState::Running(vec!["rust".to_string(), "Python".to_string()]);
    let json = serde_json::to_string(&state)?;
    println!("{:?}", json);
    Ok(())
}
impl Serialize for User {
    fn serialize<S>(&self, serialize: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serialize.serialize_struct("User", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("dob", &self.dob)?;
        state.serialize_field("skills", &self.skills)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("User", &["name", "age", "dob", "skills"], UserVisitor)
    }
}
struct UserVisitor;
impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("struct User")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let name = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let age = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        let dob = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
        let skills = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
        Ok(User {
            name,
            age,
            dob,
            skills,
        })
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut name = None;
        let mut age = None;
        let mut dob = None;
        let mut skills = None;
        let mut map = map;
        while let Some(key) = map.next_key()? {
            match key {
                "name" => {
                    if name.is_some() {
                        return Err(serde::de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                "age" => {
                    if age.is_some() {
                        return Err(serde::de::Error::duplicate_field("age"));
                    }
                    age = Some(map.next_value()?)
                }
                "dob" => {
                    if dob.is_some() {
                        return Err(serde::de::Error::duplicate_field("dob"));
                    }
                    dob = Some(map.next_value()?)
                }
                "skills" => {
                    if skills.is_some() {
                        return Err(serde::de::Error::duplicate_field("skills"));
                    }
                    skills = Some(map.next_value()?)
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }
        let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
        let age = age.ok_or_else(|| serde::de::Error::missing_field("age"))?;
        let dob = dob.ok_or_else(|| serde::de::Error::missing_field("dob"))?;
        let skills = skills.ok_or_else(|| serde::de::Error::missing_field("skills"))?;
        Ok(User {
            name,
            age,
            dob,
            skills,
        })
    }
}
