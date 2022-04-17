use crate::Destination;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serenity::client::{ClientBuilder, Context, EventHandler};
use serenity::futures::StreamExt;
use serenity::model::id::ChannelId;
use serenity::model::prelude::{Message, Ready};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct RawYoinker<T: Send + Sync + DeserializeOwned + 'static, Y: Yoinker<T>> {
    yoinker: Arc<Y>,
    destination: Destination,
    _options: YoinkOptions,
    _inner: PhantomData<T>,
}

impl<T: Send + Sync + DeserializeOwned + 'static, Y: Yoinker<T>> RawYoinker<T, Y> {
    pub fn new(yoinker: Arc<Y>, destination: Destination, _options: YoinkOptions) -> Self {
        Self {
            yoinker,
            destination,
            _options,
            _inner: PhantomData,
        }
    }

    async fn process_message(&self, msg: Message) {
        let t: T = match serde_json::from_str(&msg.content) {
            Ok(parsed) => parsed,
            Err(_) => {
                log::warn!(
                    "We couldn't parse {} into the target structure! Dropping message.",
                    msg.content
                );
                return;
            }
        };
        self.yoinker.on_message(t).await;
    }
}

#[async_trait]
pub trait Yoinker<T: DeserializeOwned + Send + Sync + 'static> {
    async fn on_message(&self, data: T);

    fn register(
        self,
        builder: ClientBuilder,
        destination: Destination,
        options: YoinkOptions,
    ) -> ClientBuilder
    where
        Self: Sized + Sync + Send + 'static,
    {
        let raw_yoinker = RawYoinker::new(Arc::new(self), destination, options);
        builder.event_handler(raw_yoinker)
    }
}

#[async_trait]
impl<T: Send + Sync + DeserializeOwned + 'static, Y: Yoinker<T> + Send + Sync + 'static>
    EventHandler for RawYoinker<T, Y>
{
    async fn ready(&self, ctx: Context, _ready: Ready) {
        match self.destination {
            Destination::Channel(channel_id) => {
                let mut stream = ChannelId(channel_id).messages_iter(&ctx.http).boxed();
                while let Some(msg_res) = stream.next().await {
                    if let Ok(msg) = msg_res {
                        self.process_message(msg).await;
                    }
                }
            }
        }
    }
    async fn message(&self, _ctx: Context, msg: Message) {
        match self.destination {
            Destination::Channel(channel_id) => {
                if channel_id != msg.channel_id.0 {
                    return;
                }
            }
        }
        self.process_message(msg).await
    }
}

pub struct YoinkOptions;
