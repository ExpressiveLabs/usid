use std::fmt::Display;
use uuid::Uuid;
use anyhow::{bail, Result};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

#[derive(Default, Clone, Copy, Debug, Eq, Hash)]
pub struct USID([u8; 16]);

impl USID {
    pub fn new() -> Self {
        Self([0; 16])
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid.into_bytes().clone())
    }

    pub fn from_bytes(bytes: &[u8; 16]) -> Self {
        Self(*bytes)
    }

    pub fn from_string(s: &str) -> Result<Self> {
        let data = if s.starts_with("usid:") {
            // Extract the fallback UUID and data from the string
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() < 2 {
                bail!("Invalid USID format")
            }
            let hash = parts[1];
            hash
        } else {
            &s
        };

        let undashed = data.replace("-", "");

        let data: [u8; 16] = undashed.as_bytes()[..16].try_into()?;
        Ok(Self(data))
    }

    pub fn as_string(&self) -> String {
        let str = String::from_utf8_lossy(&self.0);

        // Insert dashes every 4 characters
        let dashed = str.chars().enumerate().map(|(i, c)| {
            if i > 0 && i % 4 == 0 {
                format!("-{}", c)
            } else {
                c.to_string()
            }
        }).collect::<String>();

        format!("usid:{}", dashed)
    }

    pub fn as_uuid(&self) -> Uuid {
        Uuid::from_slice(&self.0).unwrap_or(Uuid::nil())
    }

    pub fn is_empty(&self) -> bool {
        self.0 == [0; 16]
    }
}

// Implement a serde serializer for USID
#[cfg(feature = "serde")]
impl Serialize for USID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state = serializer.serialize_str(&self.as_string())?;
        Ok(state)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for USID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // First, check if the USID identifier is present ("usid:...")
        let usid_str: String = Deserialize::deserialize(deserializer)?;
        USID::from_string(&usid_str).map_err(serde::de::Error::custom)
    }
}

impl PartialEq for USID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for USID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.as_string())
    }
}