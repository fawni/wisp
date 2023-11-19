use crate::serenity::{
    json::{self, Value},
    AttachmentType, CacheHttp, Context, EditMessage, Message, ReactionType, Typing,
};
use crate::{sources::tiktok::Tiktok, Error};

pub fn matches(message: &Message) -> bool {
    let re = Tiktok::valid_urls();
    re[0].is_match(&message.content) || re[1].is_match(&message.content)
}

pub async fn reembed(ctx: Context, msg: Message) -> Result<(), Error> {
    let re = Tiktok::valid_urls();
    if !re[0].is_match(&msg.content) && !re[1].is_match(&msg.content) {
        return Ok(());
    };

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::custom(|attempt| attempt.stop()))
        .build()?;

    let mut content = msg.content.clone();
    if re[1].is_match(&content) {
        let url = &re[1].captures(&content).unwrap()[0];
        let res = client.get(url).send().await?;
        content = res.headers()["location"].to_str()?.to_owned();
    }
    let aweme_id = &re[0].captures(&content).unwrap()[1];
    let location = if let Some(channel) = msg.channel(&ctx).await?.guild() {
        format!(
            "#{}, {} ({})",
            channel.name(),
            channel.guild(&ctx).unwrap().name,
            channel.guild_id
        )
    } else {
        format!("{}'s dms", msg.author.tag())
    };

    twink::mrrr!(
        "Tiktok <bold>{}</> by <bold>{}</> (<italic>{}</>) in <purple>{}</>",
        aweme_id,
        msg.author.tag(),
        msg.author.id,
        location
    );

    let Ok(tiktok) = Tiktok::from(aweme_id).await else {
        msg.react(ctx, ReactionType::Unicode(String::from("❌")))
            .await?;

        return Err("Invalid TikTok ID".into());
    };

    // TODO(upstream): use the `suppress_embeds` method - https://github.com/serenity-rs/serenity/pull/2582
    // msg.suppress_embeds(ctx).await?;
    let mut suppress = EditMessage::default();
    suppress.suppress_embeds(true);
    let map = json::hashmap_to_json_map(suppress.0);
    ctx.http()
        .edit_message(msg.channel_id.0, msg.id.0, &Value::from(map))
        .await?;

    let typing = Typing::start(ctx.http.clone(), msg.channel_id.0)?;

    let file = client.get(tiktok.video_url).send().await?.bytes().await?;

    msg.channel_id
        .send_message(&ctx, |m| {
            m.add_file(AttachmentType::Bytes {
                data: file.as_ref().into(),
                filename: format!("{aweme_id}.mp4"),
            })
            .embed(|e| {
                e.author(|a| {
                    a.name(format!(
                        "{} (@{})",
                        tiktok.author.name, tiktok.author.username
                    ))
                    .url(format!("https://tiktok.com/@{}", tiktok.author.username))
                    .icon_url(tiktok.author.avatar_url())
                })
                .description(tiktok.description)
                .field("Likes", tiktok.statistics.likes(), true)
                .field("Comments", tiktok.statistics.comments(), true)
                .field("Views", tiktok.statistics.views(), true)
                .color(0xF82_054)
            })
            .reference_message(&msg)
            .allowed_mentions(|am| am.empty_parse())
        })
        .await?;
    let _ = typing.stop();

    Ok(())
}
