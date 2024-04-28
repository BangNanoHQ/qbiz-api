use serde::Serializer;
use uuid::Uuid;

// serde custom serializer for serializing to uuid format that is lower cased and without dashes
pub fn serialize_simple_opt_uuid<S>(id: &Option<Uuid>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
  match id {
      None => s.serialize_none(),
      Some(v) => s.serialize_some(&v.to_simple().to_string().to_lowercase()),
  }
}

// serde custom serializer for serializing to uuid format that is lower cased and without dashes
pub fn serialize_simple_uuid<S>(id: &Uuid, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
     s.serialize_some(&id.to_simple().to_string().to_lowercase())
}

// test for custom serializer
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MyUuid{
      #[serde(serialize_with = "serialize_simple_uuid")]
      pub id: Uuid
    }

    #[test]
    fn test_serialize_simple_uuid() {
        // uuid strin
        let uuid_str = "9ea5da7d53e256f1ae92c4dcde993bc1";
        // parse uuid into MyUuid struct
        let my_uuid = MyUuid{
          id: Uuid::parse_str(uuid_str).unwrap()
        };

        // serialize MyUuid struct
        let serialized = serde_json::to_string(&my_uuid).unwrap();
        // expected serialized value should be the same as the uuid string

        let expected = "{\"id\":\"9ea5da7d53e256f1ae92c4dcde993bc1\"}";
        assert_eq!(serialized, expected.to_string());

        
    }

    // test deserialize
    #[test]
    fn test_deserialize_simple_uuid() {

        let json_str = "{\"id\":\"9ea5da7d53e256f1ae92c4dcde993bc1\"}";
        let my_uuid: MyUuid = serde_json::from_str(json_str).unwrap();

        let uuid_str = "9ea5da7d53e256f1ae92c4dcde993bc1";

        let expected_id = Uuid::from_str(uuid_str).unwrap();

        assert_eq!(my_uuid.id, expected_id);
        
    }
}