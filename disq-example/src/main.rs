use disq::yeet::{Destination, YeetOptions, YeeterBuilder};
use serenity::client::Client;

const TOKEN: &str = "BOT_TOKEN";

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let token = std::env::var(TOKEN).expect("Could not load the bot token!");
    let builder = Client::builder(token);

    let (builder, recv) = YeeterBuilder::<&str>::register(builder, Destination, YeetOptions);

    let handle = tokio::spawn(async move {
        let mut client = builder.await.expect("Could not build the client!");
        println!("Starting up the bot...");
        if let Err(err) = client.start().await {
            println!("Whoops, that was an error! {:?}", err);
        }
    });

    let yeeter = recv.recv().expect("Could not get the passed back Yeeter");
    yeeter.yeet("From main.rs:28!").await.expect("whoops");

    if let Err(err) = handle.await {
        log::error!("Whoops: {:?}", err);
    };
}
