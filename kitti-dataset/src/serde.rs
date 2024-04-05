pub mod occlusion {
    use crate::object::Occlusion;
    use num_traits::FromPrimitive;
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(occlusion: &Option<Occlusion>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match occlusion {
            Some(occlusion) => *occlusion as u8 as i8,
            None => -1,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Occlusion>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i8::deserialize(deserializer)?;
        let error = || D::Error::custom(format!("invalid occlusion value {value}"));

        let mode = if value < 0 {
            if value != -1 {
                return Err(error());
            }
            None
        } else {
            let occ = Occlusion::from_u8(value as u8).ok_or_else(error)?;
            Some(occ)
        };

        Ok(mode)
    }
}

pub mod object_truncation {
    use crate::object::Truncation;
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Truncation>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => f64::from(*value),
            None => -1.0,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Truncation>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        let error = || D::Error::custom(format!("invalid truncation value {value}"));

        let truncation = if value < 0.0 {
            if value != -1.0 {
                return Err(error());
            }
            None
        } else {
            let value: Truncation = value.try_into().map_err(|_| error())?;
            Some(value)
        };

        Ok(truncation)
    }
}

pub mod tracking_truncation {
    use crate::tracking::Truncation;
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Truncation>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(Truncation::Ignored) => -2.0,
            Some(Truncation::Labeled(value)) => value.raw(),
            None => -1.0,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Truncation>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        let error = || D::Error::custom(format!("invalid truncation value {value}"));

        let truncation = if value < 0.0 {
            if value != -1.0 {
                return Err(error());
            }
            None
        } else if value == 2.0 {
            Some(Truncation::Ignored)
        } else {
            let value: Truncation = value.try_into().map_err(|_| error())?;
            Some(value)
        };

        Ok(truncation)
    }
}

pub mod u8_as_f64 {
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

pub mod mode {
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
