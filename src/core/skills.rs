//! All skills are rooted in attributes: either singular attributes,
//! or a combination of multiple attributes. Knife fighting is principally
//! a matter of agility and speed, whereas archery is a function of fine
//! motor skills and vision.
//!
//! A skill may also confer a bonus for another skill. For example, cooking
//! confers a bonus to knife skills, and has reciprocal bonuses from plant
//! and animal identification.
//!
//! Every action relates to a skill, either directly or indirectly.
//! For example, "cook bread," is directly related to the cooking
//! skill, whereas "fight wombat" is dependent on the dominant weapon
//! the player is wielding.
//!
//! Magic is just another skill, with a relationship to the nature of
//! the magic. For example, a fireball depends on mastery of fire, and
//! divination a mastery of time.

use std::convert::TryFrom;

/// Mob attributes are based on a range of 0 to 255, inclusive. Increasing
/// the value has exponential cost, and the expression is linear.
/// 10 is considered childlike, 50 a healthy but unskilled adult, 200 a
/// natural maximum, and above that supernatural.
#[derive(Debug)]
pub struct Attributes {
    // Basic senses
    vision: u8,
    taste: u8,
    smell: u8,
    hearing: u8,
    touch: u8, // Fine motor skills
    // Physical attributes
    strength: u8, // Raw body strength
    speed: u8,    // Physical reflexes
    agility: u8,  // Gross motor skills
    stamina: u8,  // Ability to maintain physical exertion
    weight: u8,   // Measured in KG
    height: u8,   // Measured in CM
    // Mental attributes
    memory: u8,   // Recall of facts and experiences
    analysis: u8, // Figuring things out in the moment
    emotions: u8, // Identifying and managing emotions
    focus: u8,    // Ability to maintain mental exertion
    // Magic attributes
    magic: u8, // Ability to interact with magic energies
}

impl Attributes {
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
        match normalize(value).as_ref() {
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

#[derive(Debug, Default)]
pub struct Skill {
    name: String,
    attributes: Vec<Attribute>,
    contributors: Vec<String>, // list of other skills that contribute to this skill
}

impl Skill {
    pub fn new(name: &str) -> Self {
        Self {
            name: normalize(name),
            attributes: vec![],
            contributors: vec![],
        }
    }
}

fn normalize(input: &str) -> String {
    input.trim().to_uppercase()
}
