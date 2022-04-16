//! `yeet` is the action of enqueueing a message onto the message queue. In other
//! message brokers, you can view this action similarly to the role of a Publisher.

use crate::errors::{DisqError, Result};
use async_trait::async_trait;
use serenity::client::ClientBuilder;
use serenity::client::{Context, EventHandler};
use serenity::http::client::Http;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Ready;
use std::marker::PhantomData;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Yeeter<T> {
    http_client: Arc<Http>,
    destination: Destination,
    options: YeetOptions,
    _inner: PhantomData<T>,
}

#[derive(Debug)]
pub struct YeeterBuilder<T> {
    destination: Destination,
    options: YeetOptions,
    send: Mutex<Sender<Yeeter<T>>>,
    _inner: PhantomData<T>,
}

impl<T: Sync + Send + ToString + 'static> YeeterBuilder<T> {
    pub fn new(destination: Destination, options: YeetOptions, send: Sender<Yeeter<T>>) -> Self {
        YeeterBuilder {
            destination,
            options,
            send: Mutex::new(send),
            _inner: PhantomData,
        }
    }

    pub fn register(
        builder: ClientBuilder,
        destination: Destination,
        options: YeetOptions,
    ) -> (ClientBuilder, Receiver<Yeeter<T>>) {
        let (send, recv) = mpsc::channel();
        let builder = builder.event_handler(YeeterBuilder::<T>::new(destination, options, send));
        (builder, recv)
    }
}

impl<T: ToString> Yeeter<T> {
    pub fn new(http_client: Arc<Http>, destination: Destination, options: YeetOptions) -> Self {
        Self {
            http_client,
            destination,
            options,
            _inner: PhantomData,
        }
    }
    pub async fn yeet(&self, data: T) -> Result<()> {
        ChannelId(964704258517766218)
            .send_message(&self.http_client, |msg| msg.content(data.to_string()))
            .await
            .unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct YeetOptions;

#[derive(Debug, Clone)]
pub struct Destination;

#[async_trait]
impl<T: Send + Sync + ToString> EventHandler for YeeterBuilder<T> {
    async fn ready(&self, ctx: Context, _data: Ready) {
        let send = self.send.lock().unwrap();
        let yeeter = Yeeter::<T>::new(
            Arc::clone(&ctx.http),
            self.destination.clone(),
            self.options.clone(),
        );
        send.send(yeeter)
            .expect("Could not pass back the constructed Yeeter onReadyEvent!");
    }
}
