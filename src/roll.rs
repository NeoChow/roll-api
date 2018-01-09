use chrono::DateTime;
use chrono::prelude::Utc;
use die::Die;
use die::DieType;
use uuid::Uuid;

// Rolls all the arguments into a single struct
pub struct ComposedRoll {
    pub advantage: bool,
    pub comment: Option<String>,
    pub d: i16,
    pub die: DieType,
    pub disadvantage: bool,
    pub e: i16,
    pub gt: u16,
    pub gte: u16,
    pub h: i16,
    pub l: i16,
    pub lt: u16,
    pub lte: u16,
    pub max: i16,
    pub min: i16,
    pub modifiers: Vec<i16>,
    pub n: i16,
    pub ro: i16,
    pub rr: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Roll {
    /// Unique identifier for the roll
    pub id: String,

    /// Roll comment
    pub comment: Option<String>,

    /// The dice that compose this roll
    pub dice: Vec<Die>,

    /// Calculated equation of the roll
    pub equation: String,

    /// Time to execute final output
    pub execution_time: u64,

    /// Modifiers to apply to the combined value
    pub modifiers: Vec<i16>,

    /// The combined value of the die before modifiers
    pub raw_value: i32,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// The final combined value of the die after modifiers
    pub value: i32,
}

impl Roll {
    pub fn new(mut dice: Vec<Die>) -> Roll {
        let timestamp = Utc::now();

        // Roll each dice
        for die in &mut dice {
            die.roll();
        }

        let value = dice.iter().fold(0, |sum, d| sum + d.value as i32);

        Roll {
            comment: None,
            dice,
            equation: "".to_string(), // @refactor ideally we should be setting this on new
            timestamp,
            execution_time: 0,
            id: Uuid::new_v4().to_string(),
            modifiers: Vec::new(),
            raw_value: value,
            value,
        }
    }

    /// Associate this roll with a comment
    pub fn add_comment(&mut self, comment: String) {
        self.comment = Some(comment)
    }

    /// Associate this roll with a comment
    pub fn add_equation(&mut self, equation: String) {
        self.equation = equation
    }

    /// Add a modifier to the roll
    pub fn apply_modifier(&mut self, modifier: i16) {
        self.modifiers.push(modifier);
        self.value += modifier as i32;
    }

    /// Keep the dice greater than a number
    pub fn keep_greater_than(&mut self, keep: u16) {
        for die in &mut self.dice {
            if (die.value as u16) <= keep {
                die.drop();
                self.value -= die.value as i32;
            } else {
                die.success();
            }
        }
    }

    /// Keep the dice greater than or equal to a number
    pub fn keep_greater_than_or_equal_to(&mut self, keep: u16) {
        for die in &mut self.dice {
            if (die.value as u16) < keep {
                die.drop();
                self.value -= die.value as i32;
            } else {
                die.success();
            }
        }
    }

    /// Keep the dice less than a number
    pub fn keep_less_than(&mut self, keep: u16) {
        for die in &mut self.dice {
            if (die.value as u16) >= keep {
                die.drop();
                self.value -= die.value as i32;
            } else {
                die.success();
            }
        }
    }

    /// Keep the dice less than or equal to a number
    pub fn keep_less_than_or_equal_to(&mut self, keep: u16) {
        for die in &mut self.dice {
            if (die.value as u16) > keep {
                die.drop();
                self.value -= die.value as i32;
            } else {
                die.success();
            }
        }
    }

    /// Keep the highest rolled dice
    pub fn keep_high(&mut self, keep: u16) {
        // Sort the dice by value, drop everything below the keep value
        let mut count = 0;
        self.dice.sort_by(|a, b| b.value.cmp(&a.value));
        for die in &mut self.dice {
            if count >= keep {
                die.drop();
                self.value -= die.value as i32;
            }
            count += 1;
        }
        // sort by timestamp again before finishing the method
        self.dice.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    /// Keep the lowest rolled dice
    pub fn keep_low(&mut self, keep: u16) {
        // Sort the dice by value, drop everything below the keep value
        let mut count = 0;
        self.dice.sort_by(|a, b| a.value.cmp(&b.value));
        for die in &mut self.dice {
            if count >= keep {
                die.drop();
                self.value -= die.value as i32;
            }
            count += 1;
        }
        // sort by timestamp again before finishing the method
        self.dice.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    /// Reroll dice one time that are above a certain threshold
    pub fn reroll_dice_once_above(&mut self, threshold: i16) {
        let mut new_dice = Vec::new();
        for die in &mut self.dice {
            if !die.is_rerolled && die.value >= threshold {
                let mut d = Die::new(die.die);
                d.roll();
                let value = d.value;
                self.value += value as i32;
                &die.rerolled(&d);
                new_dice.push(d);
            }
        }

        self.dice.append(&mut new_dice);
    }

    /// Reroll dice one time that are below a certain threshold
    pub fn reroll_dice_once_below(&mut self, threshold: i16) {
        let mut new_dice = Vec::new();
        for die in &mut self.dice {
            if !die.is_rerolled && die.value <= threshold {
                let mut d = Die::new(die.die);
                d.roll();
                let value = d.value;
                self.value += value as i32;
                &die.rerolled(&d);
                new_dice.push(d);

            }
        }

        self.dice.append(&mut new_dice);
    }

    /// Reroll dice forever that are above a certain threshold (e.g. Exploding Dice)
    pub fn reroll_dice_forever_above(&mut self, threshold: i16) {
        // Reroll any dice that need to be rerolled
        self.reroll_dice_once_above(threshold);

        let mut has_more = false;
        for die in self.dice.iter() {
            if !die.is_rerolled && die.value >= threshold {
                has_more = true
            }
        }
        if has_more {
            self.reroll_dice_forever_above(threshold);
        }
    }

    /// Reroll dice forever that are below a certain threshold
    pub fn reroll_dice_forever_below(&mut self, threshold: i16) {
        // Reroll any dice that need to be rerolled
        self.reroll_dice_once_below(threshold);

        let mut has_more = false;
        for die in self.dice.iter() {
            if !die.is_rerolled && die.value <= threshold {
                has_more = true
            }
        }
        if has_more {
            self.reroll_dice_forever_below(threshold);
        }
    }
}
