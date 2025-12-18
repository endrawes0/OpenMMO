use crate::db::models::Character;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("{field} must be non-negative")]
    Negative { field: &'static str },
    #[error("{field} out of range")]
    Overflow { field: &'static str },
}

#[derive(Debug, Clone)]
pub struct CharacterWireView {
    pub level: u32,
    pub experience: u64,
    pub health: u32,
    pub max_health: u32,
    pub resource_value: u32,
    pub max_resource: u32,
}

impl TryFrom<&Character> for CharacterWireView {
    type Error = ConversionError;

    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        let level = to_u32(character.level, "level")?;
        let experience = to_u64_from_i64(character.experience, "experience")?;
        let health = to_u32(character.health, "health")?;
        let max_health = to_u32(character.max_health, "max_health")?;
        let resource_value = to_u32(character.resource_value, "resource_value")?;
        let max_resource = to_u32(character.max_resource, "max_resource")?;

        Ok(Self {
            level,
            experience,
            health,
            max_health,
            resource_value,
            max_resource,
        })
    }
}

fn to_u32(value: i32, field: &'static str) -> Result<u32, ConversionError> {
    if value < 0 {
        return Err(ConversionError::Negative { field });
    }
    u32::try_from(value).map_err(|_| ConversionError::Overflow { field })
}

fn to_u64_from_i64(value: i64, field: &'static str) -> Result<u64, ConversionError> {
    if value < 0 {
        return Err(ConversionError::Negative { field });
    }
    u64::try_from(value).map_err(|_| ConversionError::Overflow { field })
}
