use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serenity::client::{ClientBuilder, Context, EventHandler};
use serenity::model::prelude::Message;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct RawYoinker<T: Send + Sync + DeserializeOwned + 'static, Y: Yoinker<T>> {
    yoinker: Arc<Y>,
    options: YoinkOptions,
    _inner: PhantomData<T>,
}

impl<T: Send + Sync + DeserializeOwned + 'static, Y: Yoinker<T>> RawYoinker<T, Y> {
    pub fn new(yoinker: Arc<Y>, options: YoinkOptions) -> Self {
        Self {
            yoinker,
            options,
            _inner: PhantomData,
        }
    }

    pub fn register(builder: ClientBuilder, options: YoinkOptions) -> (ClientBuilder, Self) {
        unimplemented!()
    }
}

#[async_trait]
pub trait Yoinker<T: DeserializeOwned + Send + Sync + 'static> {
    async fn on_message(&self, data: T);

    fn register(self, builder: ClientBuilder, options: YoinkOptions) -> ClientBuilder
    where
        Self: Sized + Sync + Send + 'static,
    {
        let raw_yoinker = RawYoinker::new(Arc::new(self), options);
        builder.event_handler(raw_yoinker)
    }
}

#[async_trait]
impl<T: Send + Sync + DeserializeOwned + 'static, Y: Yoinker<T> + Send + Sync + 'static>
    EventHandler for RawYoinker<T, Y>
{
    async fn message(&self, _ctx: Context, msg: Message) {
        let t: T = match serde_json::from_str(&msg.content) {
            // TODO: use ?
            Ok(parsed) => parsed,
            Err(_) => panic!("We couldn't parse {} into the structure!", msg.content), // TODO: nack
        };
        self.yoinker.on_message(t).await;
    }
}

pub struct YoinkOptions;
