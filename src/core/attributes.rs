use crate::core::Skill;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

/// Mob attributes are based on a range of 0 to 255, inclusive. Increasing
/// the value has exponential cost, and the expression is linear.
/// 10 is considered childlike, 50 a healthy but unskilled adult, 200 a
/// natural maximum, and above that supernatural.
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Attributes {
    // Basic senses
    pub vision: u8,
    pub taste: u8,
    pub smell: u8,
    pub hearing: u8,
    pub touch: u8, // Fine motor skills
    // Physical attributes
    pub strength: u8, // Raw body strength
    pub speed: u8,    // Physical reflexes
    pub agility: u8,  // Gross motor skills
    pub stamina: u8,  // Ability to maintain physical exertion
    pub weight: u8,   // Measured in KG
    pub height: u8,   // Measured in CM
    // Mental attributes
    pub memory: u8,   // Recall of facts and experiences
    pub analysis: u8, // Figuring things out in the moment
    pub emotions: u8, // Identifying and managing emotions
    pub focus: u8,    // Ability to maintain mental exertion
    // Magic attributes
    pub magic: u8, // Ability to interact with magic energies
}

impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, attr: &Attribute) -> u8 {
        match attr {
            Attribute::Vision => self.vision,
            Attribute::Taste => self.taste,
            Attribute::Smell => self.smell,
            Attribute::Hearing => self.hearing,
            Attribute::Touch => self.touch,
            Attribute::Strength => self.strength,
            Attribute::Speed => self.speed,
            Attribute::Agility => self.agility,
            Attribute::Stamina => self.stamina,
            Attribute::Weight => self.weight,
            Attribute::Height => self.height,
            Attribute::Memory => self.memory,
            Attribute::Analysis => self.analysis,
            Attribute::Emotions => self.emotions,
            Attribute::Focus => self.focus,
            Attribute::Magic => self.magic,
        }
    }

    pub fn ability(&self, skill: &Skill) -> u8 {
        // the average of the attribute values is the base score for this attribute
        let mut sum = 0;
        for attr in skill.attributes.iter() {
            sum += self.get(attr) as usize;
        }
        let base = sum / skill.attributes.len();

        // the total score is the base plus the earned proficiency
        let total = base + skill.proficiency as usize;

        // ensure our total is within bounds
        if total >= (u8::MAX as usize) {
            u8::MAX
        } else {
            total as u8
        }
    }
}

#[derive(Debug)]
pub enum Attribute {
    Vision,
    Taste,
    Smell,
    Hearing,
    Touch,
    Strength,
    Speed,
    Agility,
    Stamina,
    Weight,
    Height,
    Memory,
    Analysis,
    Emotions,
    Focus,
    Magic,
}

impl TryFrom<&str> for Attribute {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match crate::normalize_str(value).as_ref() {
            "VISION" => Ok(Attribute::Vision),
            "TASTE" => Ok(Attribute::Taste),
            "SMELL" => Ok(Attribute::Smell),
            "HEARING" => Ok(Attribute::Hearing),
            "TOUCH" => Ok(Attribute::Touch),
            "STRENGTH" => Ok(Attribute::Strength),
            "SPEED" => Ok(Attribute::Speed),
            "AGILITY" => Ok(Attribute::Agility),
            "STAMINA" => Ok(Attribute::Stamina),
            "WEIGHT" => Ok(Attribute::Weight),
            "HEIGHT" => Ok(Attribute::Height),
            "MEMORY" => Ok(Attribute::Memory),
            "ANALYSIS" => Ok(Attribute::Analysis),
            "EMOTIONS" => Ok(Attribute::Emotions),
            "FOCUS" => Ok(Attribute::Focus),
            "MAGIC" => Ok(Attribute::Magic),
            e => Err(format!("Can't recognize {} as an Attribute", e)),
        }
    }
}
