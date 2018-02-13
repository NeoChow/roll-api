use chrono::DateTime;
use chrono::prelude::Utc;
use die::Die;
use die::DieType;
use uuid::Uuid;

// Rolls all the arguments into a single struct
pub struct RollFlags {
    pub die: DieType,
    pub equation: String,
    pub gt: u16,
    pub gte: u16,
    pub kh: i16,
    pub kl: i16,
    pub lt: u16,
    pub lte: u16,
    pub max: i16,
    pub min: i16,
    pub modifiers: Vec<i16>,
    pub n: i16,
    pub ro: i16,
    pub rr: i16,
    pub sides: Option<Vec<i16>>,
}

impl RollFlags {
    pub fn new() -> RollFlags {
        RollFlags {
            die: DieType::Other,
            equation: "".to_string(),
            gt: 0,
            gte: 0,
            kh: 0,
            kl: 0,
            lt: 0,
            lte: 0,
            max: 0,
            min: 1,
            modifiers: vec![],
            n: 0,
            ro: 0,
            rr: 0,
            sides: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Roll {
    /// Unique identifier for the roll
    pub id: String,

    /// The dice that compose this roll
    pub dice: Vec<Die>,

    /// Calculated equation of the roll
    pub equation: String,

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
    pub fn new(flags: RollFlags) -> Roll {
        let mut dice = vec![];
        for _ in 0..flags.n {
            let mut die = Die::new(flags.die);
            die.set_min(flags.min);
            die.set_max(flags.max);

            match flags.sides {
                Some(ref sides) => { die.sides = Some(sides.clone()); }
                None => {}
            };

            dice.push(die);
        }

        // Roll each dice
        for die in &mut dice {
            die.roll();
        }

        let value = dice.iter().fold(0, |sum, d| sum + d.value as i32);

        let mut roll = Roll {
            dice,
            equation: flags.equation,
            timestamp: Utc::now(),
            id: Uuid::new_v4().to_string(),
            modifiers: Vec::new(),
            raw_value: value,
            value,
        };

        if flags.modifiers.len() > 0 {
            for i in flags.modifiers.into_iter() {
                roll.apply_modifier(i);
            }
        }

        if flags.rr > 0 {
            roll.reroll_dice_forever_above(flags.rr);
        } else if flags.rr < 0 {
            roll.reroll_dice_forever_below(flags.rr);
        } else if flags.ro > 0 {
            roll.reroll_dice_once_above(flags.ro);
        } else if flags.ro < 0 {
            roll.reroll_dice_once_below(flags.ro);
        }

        if flags.gt != 0 {
            roll.keep_greater_than(flags.gt);
        } else if flags.gte != 0 {
            roll.keep_greater_than_or_equal_to(flags.gte);
        } else if flags.lt != 0 {
            roll.keep_less_than(flags.lt);
        } else if flags.lte != 0 {
            roll.keep_less_than_or_equal_to(flags.lte);
        } else if flags.kh != 0 {
            roll.keep_high(flags.kh as u16);
        } else if flags.kl != 0 {
            roll.keep_low(flags.kl as u16);
        }

        roll
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
