use std::{env, fs::File,};
use std::io::Write;
use pulsar::{
    message::proto::command_subscribe::SubType,
    Consumer, Pulsar, TokioExecutor
};
use futures::TryStreamExt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(||String::from("pulsar://127.0.0.1:6650"));

    let topic = String::from("persistent://public/default/upper");

    let pulsar: Pulsar<_> = Pulsar::builder(addr, TokioExecutor).build().await?;

    let mut consumer: Consumer<String, _> = pulsar
        .consumer()
        .with_topic(topic)
        .with_consumer_name("text_consumer")
        .with_options(pulsar::ConsumerOptions::default())
        .with_subscription("text-sub")
        .with_subscription_type(SubType::Exclusive)
        .build()
        .await?;

    let mut counter: usize = 0;

    let mut buffer = File::create("output.txt")?;

    while let Some(msg) = consumer.try_next().await? {

        counter += 1;
        consumer.ack(&msg).await?;

        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                log::error!("could not deserialize message: {:?}", e);
                break;
            }
        };
        let _ = buffer.write(&msg.payload.data);
        let _ = buffer.write(" ".as_bytes());
        log::info!("string message: {:?}", &data);
        log::info!("Processed {} messages", counter);
    };



    println!("EXITING CONSUMER");
    Ok(())
}
