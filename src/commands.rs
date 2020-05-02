use super::deck;
use super::help;
use log::trace;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;

#[group]
#[commands(ping, help, info, controls)]
struct General;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    help::send_help(ctx, msg.channel_id);

    Ok(())
}

#[command]
fn info(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |c| c.content(format!("Shard ID: {}", ctx.shard_id)))?;
    Ok(())
}

#[command]
fn controls(ctx: &mut Context, msg: &Message) -> CommandResult {
    controls_command(&ctx, msg);
    Ok(())
}

fn controls_command(ctx: &Context, msg: &Message) -> Option<()> {
    let channel_id = msg.channel_id;
    let topic = {
        let channel_lock = msg.channel(ctx)?.guild()?;
        trace!("lock   controls command");
        let channel = &*channel_lock.read();
        channel.topic.clone()?
    };
    trace!("unlock controls command");

    let user_id = {
        let mut split = topic.split("&");
        split.next()?;
        split.next()?;
        UserId(split.next()?.parse::<u64>().ok()?)
    };

    deck::create_deck(ctx, channel_id, "Viav Controls".into(), user_id);
    Some(())
}
