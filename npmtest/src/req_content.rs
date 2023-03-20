use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fmt;

pub struct ReqContent {
    pub content: String,
}

impl<'de> Deserialize<'de> for ReqContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Content }
        
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`content`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "content" => Ok(Field::Content),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ReqContentVisitor;

        impl<'de> Visitor<'de> for ReqContentVisitor {
            type Value = ReqContent;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ReqContent")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ReqContent, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let content = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(ReqContent{content: content})
            }

            fn visit_map<V>(self, mut map: V) -> Result<ReqContent, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut content = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Content => {
                            if content.is_some() {
                                return Err(de::Error::duplicate_field("content"));
                            }
                            content = Some(map.next_value()?);
                        }
                    }
                }
                let content = content.ok_or_else(|| de::Error::missing_field("content"))?;
                Ok(ReqContent{content:content})
            }
        }

        const FIELDS: &'static [&'static str] = &["content"];
        deserializer.deserialize_struct("ReqContent", FIELDS, ReqContentVisitor)
    }
}
