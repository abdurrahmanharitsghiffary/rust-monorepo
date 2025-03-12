    use std::time::Duration;

    //ClientConfig configure kafka client
    use rdkafka::config::ClientConfig;

    use rdkafka::error::KafkaError;
    //BaseProducer → A simple Kafka producer that sends messages asynchronously.
    // BaseRecord → Represents a Kafka message (contains topic, payload, and key).
    // Duration → Used for time-related operations (e.g., polling and flushing).
    use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
    
    pub type BaseProducerType = BaseProducer;


    pub struct KafkaProducer{
        producer: BaseProducerType
    }

    impl KafkaProducer {
        // create a new producer
    pub fn new(kafka_host: &str) -> Result<Self, KafkaError> {
        match ClientConfig::new()
            .set("bootstrap.servers", kafka_host) 
            .create()
        {
            Ok(producer) => Ok(KafkaProducer { producer }),
            Err(error) => {
                Err(error)
            }
        }
    }

    // send a message
    // Kafka menggunakan event-driven mechanism untuk memproses request asinkron, jadi harus ada polling setelah mengirim pesan. Tanpa poll(), Kafka mungkin tidak akan mengirim pesan dengan baik.
    pub fn send_message(
        &self,
        topic_name: &str,
        topic_payload: &str,
        topic_key: &str,
    ) -> Result<(), KafkaError> {
        match self
            .producer
            .send(BaseRecord::to(topic_name).payload(topic_payload).key(topic_key))
        {
            Ok(_) => {
                self.producer.poll(Duration::from_millis(100)); 
                let _= self.producer.flush(Duration::from_secs(1)); 
                Ok(())
            }
            Err((error, _)) => Err(error),
        }
    }

    }



    // pub fn base_producer(kafka_host:String, server_key:String, topic_name:&str, topic_payload:&str, topic_key:&str){
    //     //create a new client for kafka
    //     let producer: BaseProducer= match ClientConfig::new()
    //     //specify the kafka broker address
    //     .set(server_key, kafka_host)
    //     .create(){
    //         Ok(new_producer)=>new_producer,
    //         Err(error)=>{
    //             println!("this is kafka error when create producer: {}",error);
    //             return ()
    //         }
    //     };

    //     producer.send(
    //         BaseRecord::to(topic_name)
    //         .payload(topic_payload)
    //         .key(topic_key),
    //     ).expect("error");

    //     for _ in 0..10 {
    //         producer.poll(Duration::from_millis(100));
    //     }

    //     let _ = producer.flush(Duration::from_secs(1));
    //     return ()
    // }



