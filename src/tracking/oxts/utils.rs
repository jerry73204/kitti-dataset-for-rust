pub mod serde_u8_as_f64 {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &u8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fval = f64::deserialize(deserializer)?;

        if fval.fract() != 0.0 {
            return Err(D::Error::custom(format!("invalid mode value {fval}")));
        }

        if fval < 0.0 {
            return Err(D::Error::custom(format!("invalid mode value {fval}")));
        }

        let ival: u8 = fval as u8;

        Ok(ival)
    }
}

pub mod serde_mode {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(mode: &Option<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match mode {
            Some(mode) => *mode as i8,
            None => -1,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fval = f64::deserialize(deserializer)?;

        if fval.fract() != 0.0 {
            return Err(D::Error::custom(format!("invalid mode value {fval}")));
        }

        let ival = fval as i8;

        let mode = if ival < 0 {
            if ival != -1 {
                return Err(D::Error::custom(format!("invalid mode value {fval}")));
            }
            None
        } else {
            Some(ival as u8)
        };

        Ok(mode)
    }
}
