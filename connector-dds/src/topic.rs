

use std::any::type_name;

use serde::{Deserialize, Serialize};



pub trait TopicType<'a>: Serialize + Deserialize<'a> + Sized{
    fn name(&self) -> String {
        type_name::<Self>().to_string().rsplit("::").last().unwrap().to_string()
    }

    fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    fn from_json(json: &'a str) -> serde_json::Result<Self> {
        serde_json::from_str::<Self>(json)
    }
}