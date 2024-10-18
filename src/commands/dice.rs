use rand::rngs::OsRng;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::Context;

#[poise::command(prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Dice to roll, e.g., 2d20 or d20"] quantity: Option<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let quantity = quantity.unwrap_or_else(|| "1d20".to_string());

    let re = regex::Regex::new(r"(?P<num>\d*)d(?P<sides>\d+)").unwrap();
    let caps = re.captures(&quantity);

    let (num_dice, sides) = match caps {
        Some(caps) => {
            let num_str = &caps["num"];
            let num_dice: u32 = if num_str.is_empty() {
                1
            } else {
                num_str.parse()?
            };
            let sides: u32 = caps["sides"].parse()?;
            (num_dice, sides)
        }
        None => {
            ctx.say("Invalid format. Please use XdY format (e.g., 2d20 or d20).")
                .await?;
            return Ok(());
        }
    };

    if sides < 2 {
        ctx.say("Invalid number of sides. Must be at least 2.")
            .await?;
        return Ok(());
    }

    let mut rng = StdRng::from_rng(OsRng).unwrap();
    let mut dice_rolls = Vec::new();
    let mut total = 0;

    for _ in 0..num_dice {
        let roll = rng.gen_range(1..=sides);
        dice_rolls.push(roll);
        total += roll;
    }

    let mut response = String::new();
    if num_dice > 1 {
        response.push_str(&format!("Rolling {}d{}: ", num_dice, sides));
        for roll in &dice_rolls {
            response.push_str(&format!("{} ", roll));
        }
        response.push_str(&format!("= **{}**", total));
    } else {
        response.push_str(&format!("Rolling d{}: **{}**", sides, total));
    }

    ctx.say(response).await?;

    Ok(())
}
