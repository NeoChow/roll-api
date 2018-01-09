use ttml::parser::parse_step_p;
use rocket_contrib::{Json, Value};
use ttml::arg::{Arg, ArgValue, RollArg};
use roll::*;
use die::*;
use std::time::Instant;

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource not found."
    }))
}

#[get("/<command>")]
pub fn roll(command: String) -> Option<Json<Roll>> {
    // Start the timer
    let start = Instant::now();

    // Parse the roll command as if we're passing it through TTML
    let input = "!roll ".to_string() + &command;
    let step_result = parse_step_p(input.as_bytes());

    if step_result.is_err() == false {
        // Get the step from the result
        let (_, step) = step_result.unwrap();

        // Compose a roll based on the steps
        let mut composed_roll = ComposedRoll {
            advantage: false,
            comment: None,
            d: 0,
            die: DieType::Other,
            disadvantage: false,
            e: 0,
            gt: 0,
            gte: 0,
            h: 0,
            l: 0,
            lt: 0,
            lte: 0,
            max: 0,
            min: 1,
            modifiers: vec![],
            n: 0,
            ro: 0,
            rr: 0,
        };

        // build the calculated equation to output with our roll
        let mut equation = "".to_owned();

        for arg in &step.args {
            if let &Arg::Roll(RollArg::N(ArgValue::Number(n))) = arg {
                composed_roll.n = n as i16;
                equation = equation + &n.to_string();
            } else if let &Arg::Roll(RollArg::D(ArgValue::Number(d))) = arg {
                composed_roll.d = d as i16;
                composed_roll.max = d as i16;
                composed_roll.die = match d {
                    100   => DieType::D100,
                    20    => DieType::D20,
                    12    => DieType::D12,
                    10    => DieType::D10,
                    8     => DieType::D8,
                    6     => DieType::D6,
                    4     => DieType::D4,
                    _     => DieType::Other,
                };

                equation = equation + &"d" + &d.to_string();
            } else if let &Arg::Roll(RollArg::H(ArgValue::Number(h))) = arg {
                composed_roll.h = h as i16;
                equation = equation + &"kh" + &h.to_string();
            } else if let &Arg::Roll(RollArg::L(ArgValue::Number(l))) = arg {
                composed_roll.l = l as i16;
                equation = equation + &"kl" + &l.to_string();
            } else if let &Arg::Roll(RollArg::GT(ArgValue::Number(gt))) = arg {
                composed_roll.gt = gt as u16;
                equation = equation + &"gt" + &gt.to_string();
            } else if let &Arg::Roll(RollArg::GTE(ArgValue::Number(gte))) = arg {
                composed_roll.gte = gte as u16;
                equation = equation + &"gte" + &gte.to_string();
            } else if let &Arg::Roll(RollArg::LT(ArgValue::Number(lt))) = arg {
                composed_roll.lt = lt as u16;
                equation = equation + &"lt" + &lt.to_string();
            } else if let &Arg::Roll(RollArg::LTE(ArgValue::Number(lte))) = arg {
                composed_roll.lte = lte as u16;
                equation = equation + &"lte" + &lte.to_string();
            } else if let &Arg::Roll(RollArg::RR(ArgValue::Number(rr))) = arg {
                composed_roll.rr = rr as i16;
                equation = equation + &"rr" + &rr.to_string();
            } else if let &Arg::Roll(RollArg::RO(ArgValue::Number(ro))) = arg {
                composed_roll.ro = ro as i16;
                equation = equation + &"ro" + &ro.to_string();
            } else if let &Arg::Roll(RollArg::ModifierPos(ArgValue::Number(mp))) = arg {
                composed_roll.modifiers.push(mp as i16);
                equation = equation + &"+" + &mp.to_string();
            } else if let &Arg::Roll(RollArg::ModifierNeg(ArgValue::Number(mn))) = arg {
                composed_roll.modifiers.push(mn as i16);
                equation = equation + &"-" + &mn.to_string();
            } else if let &Arg::Roll(RollArg::Max(ArgValue::Number(max))) = arg {
                composed_roll.max = max as i16;
                equation = equation + &"max" + &max.to_string();
            } else if let &Arg::Roll(RollArg::Min(ArgValue::Number(min))) = arg {
                composed_roll.min = min as i16;
                equation = equation + &"min" + &min.to_string();
            } else if let &Arg::Roll(RollArg::Comment(ArgValue::Text(ref n))) = arg {
                composed_roll.comment = Some(n.to_owned());
                equation = equation + &" '" + &n + &"'";
            }
        }

        // Build the custom sided die
        let mut dice = Vec::new();
        for _ in 0..composed_roll.n {
            let mut die = Die::new(composed_roll.die);
            die.set_sides(composed_roll.d as u8);
            die.set_min(composed_roll.min);
            die.set_max(composed_roll.max);
            dice.push(die);
        }
        let mut roll = Roll::new(dice);
        roll.add_equation(equation);

        if composed_roll.modifiers.len() > 0 {
            for i in composed_roll.modifiers.into_iter() {
                roll.apply_modifier(i);
            }
        }

        if composed_roll.e > 0 {
            // todo
        } else if composed_roll.e < 0 {
            // todo
        } else if composed_roll.rr > 0 {
            roll.reroll_dice_forever_above(composed_roll.rr);
        } else if composed_roll.rr < 0 {
            roll.reroll_dice_forever_below(composed_roll.rr);
        } else if composed_roll.ro > 0 {
            roll.reroll_dice_once_above(composed_roll.ro);
        } else if composed_roll.ro < 0 {
            roll.reroll_dice_once_below(composed_roll.ro);
        }

        if composed_roll.gt != 0 {
            roll.keep_greater_than(composed_roll.gt);
        } else if composed_roll.gte != 0 {
            roll.keep_greater_than_or_equal_to(composed_roll.gte);
        } else if composed_roll.lt != 0 {
            roll.keep_less_than(composed_roll.lt);
        } else if composed_roll.lte != 0 {
            roll.keep_less_than_or_equal_to(composed_roll.lte);
        } else if composed_roll.h != 0 {
            roll.keep_high(composed_roll.h as u16);
        } else if composed_roll.l != 0 {
            roll.keep_low(composed_roll.l as u16);
        }

        // Add a comment
        match composed_roll.comment {
            Some(c) => roll.add_comment(c),
            None => {}
        }

        let elapsed = start.elapsed();
        roll.execution_time = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;

        Some(Json(roll))
    } else {
        None
    }
}
