use async_trait::async_trait;
use clap::{ArgEnum, Parser};
use disq::{
    yeet::{YeetOptions, YeeterBuilder},
    yoink::YoinkOptions,
    Destination, Yeeter, Yoinker,
};
use serde::{Deserialize, Serialize};
use serenity::client::Client;
use std::time::{Duration, SystemTime};

const YEETER_TOKEN: &str = "YEETER_TOKEN";
const YOINKER_TOKEN: &str = "YOINKER_TOKEN";

#[derive(Parser, Debug)]
struct ExampleArgs {
    #[clap(short = 'm', long = "mode", arg_enum)]
    mode: Mode,
    #[clap(short = 'l', long = "log-level")]
    log_level: log::Level,
}

#[derive(ArgEnum, Clone, Debug)]
enum Mode {
    Yeeter,
    Yoinker,
    Both,
}

#[tokio::main]
async fn main() {
    let args = ExampleArgs::parse();
    let log_level = args.log_level;
    simple_logger::init_with_level(log_level).unwrap();
    let mode = args.mode;
    let destination = Destination::Channel(964704258517766218);

    match mode {
        Mode::Yeeter => run_yeeter(destination).await,
        Mode::Yoinker => run_yoinker(destination).await,
        Mode::Both => {
            futures::join!(async { run_yeeter(destination.clone()).await }, async {
                run_yoinker(destination.clone()).await
            });
            ()
        }
    }
}

async fn run_yoinker(destination: Destination) {
    let token = std::env::var(YOINKER_TOKEN).expect("Could not load the bot token!");
    let builder = Client::builder(token);

    let yoinker = Subscriber;
    let builder = yoinker.register(builder, destination, YoinkOptions);

    let handle = tokio::spawn(async move {
        let mut client = builder.await.expect("Could not build the client!");
        println!("Starting up the bot...");
        if let Err(err) = client.start().await {
            println!("Whoops, that was an error! {:?}", err);
        }
    });

    // Indefinitely loop and let the Discord framework spin
    if let Err(err) = handle.await {
        log::error!("Whoops: {:?}", err);
    };
}

async fn run_yeeter(destination: Destination) {
    let token = std::env::var(YEETER_TOKEN).expect("Could not load the bot token!");
    let builder = Client::builder(token);

    let (builder, recv) = YeeterBuilder::<Payload>::register(builder, destination, YeetOptions);

    let handle = tokio::spawn(async move {
        let mut client = builder.await.expect("Could not build the client!");
        println!("Starting up the bot...");
        if let Err(err) = client.start().await {
            println!("Whoops, that was an error! {:?}", err);
        }
    });

    let yeeter = recv.recv().expect("Could not get the passed back Yeeter");

    // Spawn a Yeeter to write every 5 seconds
    tokio::spawn(async move {
        message(&yeeter, "Hello, World!").await;
    });

    // Indefinitely loop and let the Discord framework spin
    if let Err(err) = handle.await {
        log::error!("Whoops: {:?}", err);
    };
}

async fn message(yeeter: &Yeeter<Payload>, message: &str) {
    loop {
        let payload = Payload {
            content: message.to_owned(),
            time: SystemTime::now(),
            author: "Yeeter#7197".to_owned(),
        };
        yeeter
            .yeet(payload)
            .await
            .expect("couldn't talk to discord!");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

struct Subscriber;

#[async_trait]
impl Yoinker<Payload> for Subscriber {
    async fn on_message(&self, data: Payload) {
        log::warn!("Got data: {:?}", data);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    content: String,
    time: SystemTime,
    author: String,
}
