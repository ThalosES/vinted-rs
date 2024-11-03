use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Bool(b) => Ok(Some(b)),
        Value::Number(n) => {
            let Some(i) = n.as_i64() else {
                return Err(serde::de::Error::custom(
                    "expected an integer for optional boolean field",
                ));
            };
            {
                match i {
                    0 => Ok(Some(false)),
                    1 => Ok(Some(true)),
                    _ => Err(serde::de::Error::custom(
                        "expected 0 or 1 for optional boolean field",
                    )),
                }
            }
        }

        Value::Null => Ok(None), // Handle `null` as `None`
        _ => Err(serde::de::Error::custom(
            "expected a boolean, integer, or null for optional boolean field",
        )),
    }
}
