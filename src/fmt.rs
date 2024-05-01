pub mod datetime_format {
  use chrono::NaiveDateTime;
  use serde::{self, Deserialize, Serializer, Deserializer};

  const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S"; 

  pub fn serialize<S>(
      date: &NaiveDateTime,
      serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let s = format!("{}", date.format(FORMAT));
      serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(
      deserializer: D,
  ) -> Result<NaiveDateTime, D::Error>
  where
      D: Deserializer<'de>,
  {
      let s = String::deserialize(deserializer)?;
      NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
  }
}

pub mod optional_datetime_format {
  use chrono::NaiveDateTime;
  use serde::{self, Deserialize, Serializer, Deserializer};

  const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

  pub fn serialize<S>(
      date: &Option<NaiveDateTime>,
      serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      match date {
          Some(date) => {
              let s = format!("{}", date.format(FORMAT));
              serializer.serialize_str(&s)
          },
          None => serializer.serialize_none(),
      }
  }

  pub fn deserialize<'de, D>(
      deserializer: D,
  ) -> Result<Option<NaiveDateTime>, D::Error>
  where
      D: Deserializer<'de>,
  {
      let s: Option<String> = Option::deserialize(deserializer)?;
      match s {
          Some(s) => {
              let date = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
              Ok(Some(date))
          },
          None => Ok(None),
      }
  }
}