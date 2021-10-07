use std::{env, error::Error};
use futures::stream::StreamExt;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::{Cluster, ShardScheme, Events}, Event};
use twilight_command_parser::{CommandParserConfig, Parser};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, EmbedAuthorBuilder, EmbedFooterBuilder ,ImageSource};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;
use anyhow;
use std::fs;
use serde::Deserialize;


#[derive(Clone, Copy)]
pub struct Bot(&'static BotConfig);


pub struct BotConfig {
    pub cache: InMemoryCache,
    pub cluster: Cluster,
    pub http: HttpClient,
}

#[derive(Deserialize)]
struct Token {
    token: String,
}

fn getconf() -> Result<Token, anyhow::Error> 
{
    let config = env::var("TOKEN")?;
    Ok( Token  {
        token: config
    })
}

impl Bot {
    async fn new(config: Token) -> Result<(Self, Events), anyhow::Error> {

        let http = HttpClient::new(config.token.clone());
        
        let id = http.current_user().exec().await?;

        let cache = InMemoryCache::builder()
            .resource_types(ResourceType::MESSAGE)
            .build();
        
        let scheme = ShardScheme::Auto; 
        
        let (cluster, events) = Cluster::builder(config.token.to_owned(), Intents::GUILD_MESSAGES)
        .shard_scheme(scheme)
        .build()
        .await?;
        
        
        Ok(( 
            Self(Box::leak(Box::new(BotConfig {
                cache, cluster, http
            }))), events 
        ))
    }

    async fn connect(self) {
        self.0.cluster.up().await;
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    

    let (bot, mut events) = Bot::new(getconf()?).await?;

    tokio::spawn(bot.connect());

    // Process each event as they come in.
    while let Some((shard_id, event)) = events.next().await {
        // Update the cache with the event.
        bot.0.cache.update(&event);
        tokio::spawn(handle_event(shard_id, event, bot.0.http.clone()));
    }

    Ok(())
}

async fn handle_event(shard_id: u64, event: Event, http: HttpClient) -> Result<(), Box<dyn Error + Send + Sync>> {
    let embed = EmbedBuilder::new()
        .color(0x00D6_8717)
        .description(
            "Simply, the trinity is a core doctrine that we hold too, that simply outlines the Godhood and how the divine essence is distributed to the three hypostasis. Which thus, creates the hypostatic union, that equates each hypostasis, together within essence. Christ (A), is God, whilst the Father (B) is God, and the Holy Spirit (C) being God. All sharing the common nature and essence of divine relativity. Although, one hypostasis holding a common nature with the human nature, whilst also upholding the divine nature. Which is (Christ). But, the trinity has distinct persons, which in sense, the father is distinct in reference to the son and holy spirit. All having properties and difference of person hood, which needs to be emphasized because of the heresy of modalism. We don’t want to fall into a modal collapse, which is a heresy. Or getting into Nestorianism, which is easy to fall into when discussing the hypostatic union. As Nestorius, brought up the ideal of Christ holding two natures, which is completely illogical. For Christ to hold two natures, means that he’s two beings, which undermines the trinity and hypostatic union, which evidentially falls into 3 being apart of the holy divine logos."
        )
        .author(EmbedAuthorBuilder::new().name("Trinity").icon_url(ImageSource::url("https://previews.123rf.com/images/ivandbajo/ivandbajo1903/ivandbajo190300252/118779642-trinity-logo-design-inspiration-trinity-love-logo-isolated-on-white-background.jpg")?))
        .thumbnail(ImageSource::url("https://previews.123rf.com/images/ivandbajo/ivandbajo1903/ivandbajo190300252/118779642-trinity-logo-design-inspiration-trinity-love-logo-isolated-on-white-background.jpg")?)
        .footer(EmbedFooterBuilder::new("AL Library").icon_url(ImageSource::url("https://previews.123rf.com/images/ivandbajo/ivandbajo1903/ivandbajo190300252/118779642-trinity-logo-design-inspiration-trinity-love-logo-isolated-on-white-background.jpg")?))
        .build()?;
    
    let sembed = EmbedBuilder::new()
        .title("We are saved through faith")
        .color(0x00D6_8717)
        .description("Salvation is NOT by works, but through faith. Yes, our salvation does come by grace received through faith. However, this does not mean that good works are irrelevant to salvation, our righteous works are considered filthy rags. But remember, faith with works is dead. Therefore, believe Jesus Christ and accept Him as your Lord and Savior, you will be saved.")
        .image(ImageSource::url("https://images-ext-1.discordapp.net/external/WLPDWDRVCAxjluCWyZahb6BMW2FDTdO5EN-ahouGA70/http/www.sfltimes.com/wp-content/uploads/2015/03/Just-a-Closer-Walk-with-Jesus.jpg?width=945&height=577")?)
        .build()?;
    match event {
        Event::MessageCreate(msg) if msg.content == "!trinity" => {
            http
            .create_message(msg.channel_id)
            .embeds(&[embed])?
            .exec()
            .await?;
        }
        Event::MessageCreate(msg) if msg.content == "!salvation" => {
            http
            .create_message(msg.channel_id)
            .embeds(&[sembed])?
            .exec()
            .await?;
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}
