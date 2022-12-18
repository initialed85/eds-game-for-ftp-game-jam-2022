use rmp_serde;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

//
// JSON
//

pub fn serialize_json<T>(t: T) -> String
where
    T: Serialize,
{
    return serde_json::to_string(&t).unwrap();
}

pub fn deserialize_json<T>(message: String) -> T
where
    T: DeserializeOwned,
{
    return serde_json::from_str::<T>(message.as_str()).unwrap();
}

//
// RMP
//

pub fn serialize<T>(t: T) -> Vec<u8>
where
    T: Serialize,
{
    return rmp_serde::to_vec(&t).unwrap();
}

pub fn deserialize<T>(message: Vec<u8>) -> T
where
    T: DeserializeOwned,
{
    return rmp_serde::from_slice::<T>(message.as_slice()).unwrap();
}
