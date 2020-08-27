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

#[derive(Debug)]
pub enum Attribute {
    // Physical attributes
    Vision,
    Taste,
    Smell,
    Hearing,
    Touch,    // Fine motor skills
    Strength, // Raw body strength
    Speed,    // Physical reflexes
    Agility,  // Gross motor skills
    Fitness,  // Ability to maintain physical exertion
    Weight,   // Measured in KG
    Height,   // Measured in CM
    // Mental attributes
    Memory,   // Recall of facts and experiences
    Analysis, // Figuring things out in the moment
    Emotions, // Identifying and managing emotional impulses
    Empathy,  // Identifying others' emotions
    Focus,    // Ability to maintain mental exertion
}

#[derive(Debug, Default)]
pub struct Skill {
    name: String,
    attributes: Vec<String>,
    enhances: Vec<String>,
}

impl Skill {
    pub fn new(name: &str) -> Self {
        Self {
            name: normalize(name),
            attributes: vec![],
            enhances: vec![],
        }
    }
}

fn normalize(input: &str) -> String {
    input.trim().to_uppercase()
}
