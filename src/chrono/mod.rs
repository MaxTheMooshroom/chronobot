pub mod dice;

use std::sync::Arc;

use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};

use crate::bot::{BotState, CommandContext, CommandFuture};
use crate::log::LogContext;

pub fn roll(bs: BotState, ctx: Arc<CommandContext>) -> CommandFuture<()> {
    Box::pin(async move {
        if ctx.content.is_empty() {
            return;
        }

        let lc = ctx.content.to_lowercase();
        let pools_maybe = lc.split_ascii_whitespace()
            .map(|s| s.split_once(":")
                .map_or(
                    Ok((s, 1)),

                    |(left, right)|
                        right.parse::<u8>().map(|n| (left, n))
                )
            )
            .collect();

        let pools: Vec<(&str, u8)>;
        if let Err(e) = pools_maybe {
            ctx.msg.channel_id.send_message(&ctx.ctx, CreateMessage::new().content(
                format!("an pool's rhs is not a valid utf8: {e:#?}")
            )).await;
            return;
        } else {
            pools = unsafe { pools_maybe.unwrap_unchecked() };
        }

        match dice::roll_dice(&pools) {
            Ok(r) => {
                /// SAFETY: This is safe because we already checked these
                ///         colours in dice::roll_dice()
                let names = unsafe {
                    dice::colors_to_names(&pools).unwrap_unchecked()
                };

                let rolls = names.iter().fold(
                    CreateEmbed::new().title("Rolls"),
                    |rolls, &(name, n)| rolls.field(name, n.to_string(), false)
                );

                let response = CreateMessage::new()
                    .add_embed(rolls)
                    .add_embed(
                        CreateEmbed::new()
                            .title("Results")
                            .field("Time", r.blank.to_string(), false)
                            .field("Success/Failure", r.passfail.to_string(), false)
                            .field("Opportunity/Threat", r.luck.to_string(), false)
                            .field("Triumph", r.triumph.to_string(), false)
                            .field("Despair", r.despair.to_string(), false)
                    );

                ctx.msg.channel_id.send_message(&ctx.ctx, response).await;
            },
            Err(e) => {
                ctx.msg.channel_id.say(&ctx.ctx, format!("Error: {e:#?}")).await;
            }
        };
    })
}

