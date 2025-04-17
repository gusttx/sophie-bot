use crate::{types::{Context, UnitResult}, utils::discord::{embed::Embed, reply::Reply}};
use poise::command;

/// Salve o maior de todos
#[command(
    slash_command,
    user_cooldown = 10
)]
pub async fn salve(ctx: Context<'_>) -> UnitResult {
    let embed = Embed::new(0xFF0000, "Hino do Corinthians")
        .small_image("https://a.espncdn.com/i/teamlogos/soccer/500/874.png")
        .footer("© Sophia Club Corinthians Paulista")
        .url("https://www.youtube.com/watch?v=g6M8oJq-dEA")
        .desc(
            "Salve o Corinthians
            O campeão dos campeões
            Eternamente
            Dentro dos nossos corações

            Salve o Corinthians
            De tradições e glórias mil
            Tu és orgulho
            Dos desportistas do Brasil

            Teu passado é uma bandeira
            Teu presente é uma lição
            Figuras entre os primeiros
            Do nosso esporte bretão

            Corinthians grande
            Sempre altaneiro
            És do Brasil
            O clube mais brasileiro

            Salve o Corinthians
            O campeão dos campeões
            Eternamente
            Dentro dos nossos corações

            Salve o Corinthians
            De tradições e glórias mil
            Tu és orgulho
            Dos desportistas do Brasil"
        );

    Reply::with_embed(embed).send_ok(&ctx).await
}
