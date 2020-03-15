use lazy_static::lazy_static;
use serenity::model::channel::ChannelType;
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::channel::ReactionType;
use serenity::model::permissions::Permissions;
use serenity::model::prelude::EmojiId;
use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;
use serenity::prelude::RwLock;
use serenity::utils::Colour;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref ID: Mutex<u8> = Mutex::new(250);
}

pub fn voice_create(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
    user_id: UserId,
) -> Option<()> {
    let voice_channel = voice_channel.read();
    let channel_type = ChannelType::Voice;
    guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(channel_type)
                .name::<&str>(voice_channel.name.as_ref())
                .permissions(voice_channel.permission_overwrites.clone());

            if let Some(category_id) = voice_channel.category_id {
                create_channel = create_channel.category(category_id);
            }

            if let Some(user_limit) = voice_channel.user_limit {
                create_channel = create_channel.user_limit(user_limit as u32);
            }

            create_channel
        })
        .ok()?;

    let id = {
        let mut lock = ID.lock().unwrap();
        *lock = lock.overflowing_add(1).0;
        *lock
    };

    let new_name = format!("{} / {}", voice_channel.name, id);
    let voice_channel_id = voice_channel.id;
    voice_channel_id
        .edit(ctx, |c| c.name::<&str>(new_name.as_ref()))
        .ok()?
        .create_permission(
            ctx,
            &PermissionOverwrite {
                allow: Permissions::MANAGE_CHANNELS | Permissions::MOVE_MEMBERS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .ok()?;

    let screen_share_link = format!(
        "https://discordapp.com/channels/{}/{}",
        guild_id.0, voice_channel.id.0
    );

    let channel_type = ChannelType::Text;
    guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(channel_type)
                .topic(format!("**Screen Share: {}** - &{}&{}", screen_share_link, voice_channel.id.0, user_id.0))
                .name(format!("voice-viav-{}", id))
                .permissions(voice_channel.permission_overwrites.clone());

            if let Some(category_id) = voice_channel.category_id {
                create_channel = create_channel.category(category_id);
            }
            create_channel
        })
        .ok()?
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(new_name)
                        .icon_url("https://cdn.discordapp.com/attachments/451092625894932493/681741191313883186/Viav.png")
                        .url("https://viav.app/")
                })
                .field(
                    "Video",
                    format!("[` Share Screen `]({})", screen_share_link),
                    true,
                )
                .field("Like Viav?", "[` Vote on Top.gg `](https://top.gg/bot/446151195338473485/vote)", true)
                .field("Owner", format!("<@{}>", user_id.0), true)
                .colour(Colour::from_rgb(103, 58, 183))
            })
            .reactions(vec![
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684471911920566281),
                    name: Some(String::from("lock")),
                },
                // ReactionType::Custom {
                //     animated: false,
                //     id: EmojiId(684471928739725376),
                //     name: Some(String::from("eye")),
                // },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684470685430448128),
                    name: Some(String::from("alert")),
                },
                // ReactionType::Custom {
                //     animated: false,
                //     id: EmojiId(684471126130425935),
                //     name: Some(String::from("help")),
                // },
            ])
        })
        .ok()?
        .pin(ctx)
        .ok()?;

    Some(())
}
