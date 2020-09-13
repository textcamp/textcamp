//! All skills are rooted in attributes: either singular attributes,
//! or a combination of multiple attributes. For example, knife fighting
//! is principally a matter of agility and speed, whereas archery is a
//! function of fine motor skills and vision.

use crate::core::{Attribute, Attributes};

#[derive(Debug, Default)]
pub struct Skill {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub proficiency: u8,
}

impl Skill {
    pub fn new(name: &str) -> Self {
        Self {
            name: crate::normalize_str(name),
            attributes: vec![],
            proficiency: 0,
        }
    }

    /// Attempts to perform a skill given a random value. A _low_ random value
    /// is more likely to succeed, because abilities _increase_ over time. A
    /// 0 is always going to succeed (as a CriticalSuccess) and a 255 is always
    /// going to fail (as a CriticalFail).
    pub fn attempt(&self, attributes: &Attributes, rnd: u8) -> Attempt {
        match rnd {
            u8::MIN => Attempt::CriticalSuccess,
            u8::MAX => Attempt::CriticalFail,
            _ => {
                let ability = attributes.ability(&self);
                if rnd <= ability {
                    // 🎉
                    Attempt::Success
                } else {
                    // 😢
                    Attempt::Fail
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Attempt {
    CriticalFail,
    Fail,
    Success,
    CriticalSuccess,
}
