use anyhow::Result;
use rmp_serde;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

//
// JSON
//

pub fn serialize_json<T>(t: T) -> Result<String>
where
    T: Serialize,
{
    Ok(serde_json::to_string(&t)?)
}

pub fn deserialize_json<T>(message: String) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(serde_json::from_str::<T>(message.as_str())?)
}

//
// RMP
//

pub fn serialize<T>(t: T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    Ok(rmp_serde::to_vec(&t)?)
}

pub fn deserialize<T>(message: Vec<u8>) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(rmp_serde::from_slice::<T>(message.as_slice())?)
}
