use crate::traits::serializable::BinarySerializable;

#[derive(Debug)]
pub struct Metric {
    pub timestamp: u64,
    pub name: String,
    pub labels: Vec<(String, String)>
}

impl BinarySerializable for Metric {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(&self.timestamp.to_le_bytes());
        data.extend((self.name.len() as u32).to_le_bytes());
        data.extend(self.name.as_bytes());
        for (key, value) in &self.labels {
            data.extend((key.len() as u32).to_le_bytes());
            data.extend(key.as_bytes());
            data.extend((value.len() as u32).to_le_bytes());
            data.extend(value.as_bytes());
        }
        data
    }

    fn deserialize(_data: &[u8], byte_offset: &mut usize) -> std::result::Result<Self, String> where Self: Sized {
        let timestamp = u64::from_le_bytes(_data[*byte_offset..*byte_offset + 8].try_into().unwrap());
        if timestamp == 0
        {
            return Err(String::from("Failed to deserialize metric..."))
        }

        *byte_offset += 8;
        let name_len = u32::from_le_bytes(_data[*byte_offset..*byte_offset + 4].try_into().unwrap()) as usize;
        *byte_offset += 4;
        let name = String::from_utf8(_data[*byte_offset..*byte_offset + name_len].to_vec()).unwrap();
        *byte_offset += name_len;

        let mut labels = Vec::new();
        while *byte_offset < _data.len() {
            let key_len = u32::from_le_bytes(_data[*byte_offset..*byte_offset + 4].try_into().unwrap()) as usize;
            *byte_offset += 4;
            let key = String::from_utf8(_data[*byte_offset..*byte_offset + key_len].to_vec()).unwrap();
            *byte_offset += key_len;
            let value_len = u32::from_le_bytes(_data[*byte_offset..*byte_offset + 4].try_into().unwrap()) as usize;
            *byte_offset += 4;
            let value = String::from_utf8(_data[*byte_offset..*byte_offset + value_len].to_vec()).unwrap();
            *byte_offset += value_len;

            labels.push((key, value));
        }

        Ok(Self {
            timestamp,
            name,
            labels 
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_metric_serialization() {
        let metric = Metric {
            timestamp: 1622547800,
            name: "test_metric".to_string(),
            labels: vec![("label1".to_string(), "value1".to_string()), ("label2".to_string(), "value2".to_string())]
        };

        let serialized = metric.serialize();
        let mut byte_offset: usize = 0;
        let deserialized = Metric::deserialize(&serialized, &mut byte_offset).unwrap();
        assert_eq!(deserialized.timestamp, metric.timestamp);
        assert_eq!(deserialized.name, metric.name);
        assert_eq!(deserialized.labels.len(), metric.labels.len());
        for (i, (key, value)) in deserialized.labels.iter().enumerate() {
            assert_eq!(key, &metric.labels[i].0);
            assert_eq!(value, &metric.labels[i].1);
        }
    }
}