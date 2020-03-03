use serenity::model::prelude::Reaction;
use serenity::{
    model::{
        gateway::{Activity, Ready},
        id::{ChannelId, GuildId},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler},
};

use super::deck;
use super::voice_events;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(Activity::listening("-viav "));
    }

    fn voice_state_update(
        &self,
        ctx: Context,
        guild: Option<GuildId>,
        old: Option<VoiceState>,
        new: VoiceState,
    ) {
        let guild_id = match guild {
            Some(guild_id) => guild_id,
            None => return,
        };

        let new_id = new.channel_id;
        let old_id = old.and_then(|state| state.channel_id);

        if new_id != old_id {
            if let Some(old_id) = old_id {
                if let Some(channel) = old_id.to_channel(&ctx).unwrap().guild() {
                    voice_events::on_leave(&ctx, guild_id, channel, old.unwrap().user_id);
                }
            }
            if let Some(new_id) = new_id {
                if let Some(channel) = new_id.to_channel(&ctx).unwrap().guild() {
                    voice_events::on_join(&ctx, guild_id, channel, new.user_id);
                }
            }
        }
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction) {
            deck::on_deck_reaction_add(&ctx, &reaction, &mut vc, &mut tc, owner);
        }
    }

    fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction) {
            deck::on_deck_reaction_remove(&ctx, &reaction, &mut vc, &mut tc, owner);
        }
    }
}
