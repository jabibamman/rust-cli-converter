use crate::converter::errors::ConversionError;

pub struct JsonToXmlConverter {}

impl JsonToXmlConverter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn convert_json_to_xml(&self, json: &str) -> Result<String, ConversionError> {
        let value = serde_json::from_str::<serde_json::Value>(json).map_err(|_| ConversionError::InvalidJson)?;
        let transformed_value = Self::transform_json_array(&value);
        let mut xml = serde_xml_rs::to_string(&transformed_value).map_err(|_| ConversionError::InvalidXml)?;

        xml = xml.replace("</>", "");
        xml = xml.replace("<>", "");

        Ok(xml)
    }



    fn transform_json_array(value: &serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map.iter() {
                    new_map.insert(k.clone(), Self::transform_json_array(v));
                }
                serde_json::Value::Object(new_map)
            }
            serde_json::Value::Array(array) => {
                let new_array: Vec<_> = array.iter().map(|v| Self::transform_json_array(v)).collect();
                serde_json::json!({ "item": new_array })
            }
            _ => value.clone(),
        }
    }

}