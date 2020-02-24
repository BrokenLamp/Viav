use serenity::{
    model::id::{ChannelId, GuildId},
    model::voice::VoiceState,
    prelude::{Context, EventHandler},
};

use super::voice_events;

pub struct Handler;

impl EventHandler for Handler {
    fn voice_state_update(
        &self,
        ctx: Context,
        guild: Option<GuildId>,
        old: Option<VoiceState>,
        new: VoiceState,
    ) {
        let mut ctx = ctx;
        let guild_id = match guild {
            Some(guild_id) => guild_id,
            None => return,
        };

        let new_id_num = new.channel_id.unwrap_or(ChannelId(0)).0;
        let old_id_num = match &old {
            Some(old_id) => old_id.channel_id.unwrap_or(ChannelId(0)).0,
            None => 0,
        };

        println!("{} : {}", new_id_num, old_id_num);

        if new_id_num != old_id_num {
            if old_id_num != 0 {
                voice_events::on_leave(&mut ctx, guild_id, old);
            }
            if new_id_num != 0 {
                voice_events::on_join(&mut ctx, guild_id, new);
            }
        }
    }
}
