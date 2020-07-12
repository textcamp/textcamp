use rand::prelude::*;

/// # Dice
///
/// Provides a simple interface for generating random values in the scheme of dice.
///
/// ## Examples
///
/// ```
/// # use textcamp::core::Dice;
/// # let loops = 50;
/// let mut dice = Dice::new();
///
/// # for _ in 0..loops {
/// let ranged = dice.range(0, 10);
/// assert!(ranged >= 0);
/// assert!(ranged < 10);
/// # }
///
/// # for _ in 0..loops {
/// let roll_1d20 = dice.d(1, 20);
/// assert!(roll_1d20 >= 1);
/// assert!(roll_1d20 <= 20);
/// # }
///
/// # for _ in 0..loops {
/// let roll_string = "5d6";
/// let result = dice.roll(roll_string);
/// assert!(result.is_ok());
///
/// let value = result.unwrap();
/// assert!(value >= 5);
/// assert!(value <= 5*6);
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Dice {
    rng: rand::rngs::ThreadRng,
}

impl Dice {
    /// Instantiates with a fresh copy of `thread_rng()`
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Produces a value between `low` (inclusive) and `high` (exclusive)
    pub fn range<T: rand::distributions::uniform::SampleUniform>(&mut self, low: T, high: T) -> T {
        self.rng.gen_range(low, high)
    }

    /// Rolls a `sides` sided die `count` times (eg: `d(6, 3)` rolls a six sided die three times)
    pub fn d(&mut self, sides: usize, count: usize) -> usize {
        let mut sum = 0;
        for _ in 0..count {
            sum += self.range(1, sides + 1);
        }

        sum
    }

    /// Rolls a standard roll string (eg: `roll("3d6")` rolls a six sided die three times)
    pub fn roll(&mut self, input: &str) -> Result<usize, DiceErr> {
        let mut vs: Vec<&str> = input.split('d').collect();
        let raw_sides = vs.pop().ok_or_else(|| DiceErr::BadDiceString)?;
        let raw_count = vs.pop().ok_or_else(|| DiceErr::BadDiceString)?;

        let sides = raw_sides.parse().map_err(|_| DiceErr::BadDiceString)?;
        let count = raw_count.parse().map_err(|_| DiceErr::BadDiceString)?;

        Ok(self.d(sides, count))
    }
}

#[derive(Debug)]
pub enum DiceErr {
    BadDiceString,
}
