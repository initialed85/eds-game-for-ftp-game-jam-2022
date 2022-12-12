use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize<T>(t: T) -> String
where
    T: Serialize,
{
    return serde_json::to_string(&t).unwrap();
}

pub fn deserialize<T>(message: String) -> T
where
    T: DeserializeOwned,
{
    return serde_json::from_str::<T>(message.as_str()).unwrap();
}
