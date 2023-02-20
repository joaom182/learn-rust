use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message,
};

fn get_consumer(group: &str) -> BaseConsumer {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("group.id", group)
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
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("max.poll.interval.ms", "2000")
        .set("session.timeout.ms", "1000")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        // .set("auto.offset.reset", "largest")
        .create()
        .expect("Consumer creation error");
    consumer
}

#[tokio::main]
async fn main() {
    let topic = "test";
    let group = "consumer-group";
    let consumer = get_consumer(group);
    consumer.subscribe(&[topic]).unwrap();

    loop {
        match consumer.poll(std::time::Duration::from_millis(100)) {
            Some(Ok(message)) => {
                println!("xxxx");
                println!(
                    "Received message: {}",
                    match message.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(_)) => "<invalid utf-8>",
                    }
                );
            }
            Some(Err(e)) => {
                eprintln!("Error while receiving message: {:?}", e);
            }
            None => {}
        }
    }
}
