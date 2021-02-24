use super::MASTER_USER;
use crate::channel_utils::TopicData;
use anyhow::Context;
use serenity::{
    model::prelude::{
        ChannelId, EmojiId, GuildChannel, Message, PermissionOverwrite, PermissionOverwriteType,
        Permissions, Reaction, ReactionType, RoleId, User, UserId,
    },
    utils::Colour,
};

pub async fn on_deck_reaction(
    ctx: &serenity::client::Context,
    reaction: &Reaction,
    is_add: bool,
    voice_channel: &mut GuildChannel,
    text_channel: &mut GuildChannel,
    _owner: User,
) -> anyhow::Result<()> {
    let emoji_name = match &reaction.emoji {
        ReactionType::Custom {
            animated: _,
            id: _,
            name,
        } => name.clone(),
        _ => return Ok(()),
    };

    let emoji_name = match emoji_name {
        Some(name) => name,
        None => return Ok(()),
    };

    match emoji_name.as_str() {
        "lock" => {
            voice_channel
                .edit(ctx, |e| e.user_limit(is_add as u64))
                .await
                .context("Failed to lock voice channel")?;
        }

        "eye" => {
            let permissions = if is_add {
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::READ_MESSAGES | Permissions::CONNECT,
                    kind: PermissionOverwriteType::Role(RoleId(voice_channel.guild_id.0)),
                }
            } else {
                text_channel
                    .topic
                    .as_ref()
                    .and_then(|topic| {
                        let topic_data = TopicData::from_string(topic)?;
                        Some(PermissionOverwrite {
                            allow: topic_data.allow,
                            deny: topic_data.deny,
                            kind: PermissionOverwriteType::Role(RoleId(voice_channel.guild_id.0)),
                        })
                    })
                    .unwrap_or(PermissionOverwrite {
                        allow: Permissions::READ_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Role(RoleId(voice_channel.guild_id.0)),
                    })
            };
            voice_channel
                .create_permission(ctx, &permissions)
                .await
                .context("Failed to hide voice channel")?;
        }

        "alert" => {
            text_channel
                .edit(ctx, |e| e.nsfw(is_add))
                .await
                .context("Failed to set text channel to NSFW")?;
        }

        _ => {}
    }

    Ok(())
}

pub async fn create_deck(
    ctx: &serenity::client::Context,
    channel_id: ChannelId,
    deck_name: String,
    user_id: UserId,
) -> Option<Message> {
    channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(deck_name)
                        .icon_url("https://cdn.discordapp.com/attachments/451092625894932493/681741191313883186/Viav.png")
                        .url("https://viav.app/")
                })
                .field("Channel Owner", format!("<@{}>", user_id.0), true)
                .colour(Colour::from_rgb(103, 58, 183))
            })
            .reactions(vec![
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684471911920566281),
                    name: Some(String::from("lock")),
                },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684471928739725376),
                    name: Some(String::from("eye")),
                },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684470685430448128),
                    name: Some(String::from("alert")),
                },
            ])
        })
        .await
        .ok()
}

pub async fn get_deck_reaction_info(
    ctx: &serenity::client::Context,
    reaction: &Reaction,
) -> Option<(GuildChannel, GuildChannel, User)> {
    if reaction.user(ctx).await.ok()?.bot {
        return None;
    }

    let text_channel = { reaction.channel(ctx).await.ok()?.guild()? };

    let topic = text_channel.topic.as_ref()?.clone();
    let mut topic = topic.split("&");

    topic.next()?;

    let voice_channel = {
        ChannelId(topic.next()?.parse::<u64>().ok()?)
            .to_channel(ctx)
            .await
            .ok()?
            .guild()?
    };

    let owner = UserId(topic.next()?.parse::<u64>().ok()?)
        .to_user(ctx)
        .await
        .ok()?;

    let is_channel_owner = owner.id == reaction.user_id?;
    let is_master_user = MASTER_USER == reaction.user_id?;
    let is_server_admin = {
        reaction
            .channel(ctx)
            .await
            .ok()?
            .guild()?
            .permissions_for_user(ctx, reaction.user_id?)
            .await
            .ok()?
            .manage_channels()
    };

    if is_channel_owner || is_server_admin || is_master_user {
        Some((voice_channel, text_channel, owner))
    } else {
        None
    }
}
