use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    let consumer = create_consumer("group_1", "test");
    let mut msg_stream = consumer.start();

    println!("Start listenning!");

    while let Some(msg) = msg_stream.next().await {
        match msg {
            Ok(msg) => {
                // tha payload can be empty
                println!("xxxx");
                println!(
                    "Received message: {}",
                    match msg.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(_)) => "<invalid utf-8>",
                    }
                );

                // now we can store the offset to be committed in the next auto-commit so this
                // message will never be processed again
                let res = consumer.store_offset(&msg);
                match res {
                    Ok(()) => {}
                    Err(e) => println!("Could not commit message: {} ", e),
                }
            }
            Err(e) => {
                println!("Could not receive and will not process message: {}", e)
            }
        };
    }
}

fn create_consumer(group_id: &str, topic: &str) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set(
            "bootstrap.servers",
            "sunny-crayfish-10421-us1-kafka.upstash.io:9092",
        )
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set(
            "sasl.username",
            "c3VubnktY3JheWZpc2gtMTA0MjEkzNIysmd3AsxO3gLdrnVxJinzsTuyShps9wQ",
        )
        .set("sasl.password", "3128672162cb40c09d2c3ebc657faa0a")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.commit.interval.ms", "1000")
        .set("enable.auto.offset.store", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[topic])
        .expect("Can't subscribe to specified topic");

    consumer
}
