//! AWS Lambda function for testing
//! API Gateway -> Kinesis -> Lambda integration.

use std::convert::TryFrom;
use std::error::Error;
use log::{info, warn, LevelFilter};

use aws_lambda_events::event::kinesis::{KinesisEvent, KinesisEventRecord};
use lambda_runtime::{lambda, Context};
use lambda_runtime::error::HandlerError;
use serde::Deserialize;
use simplelog::SimpleLogger;


/// The item sent by Kinesis.
/// Structure defined in `cloudformation.yaml` under the [ItemModel] resource.
#[derive(Deserialize, Debug)]
struct Item {
    pub id: i64,
    pub description: String,
    pub count: i64,
}

impl TryFrom<&KinesisEventRecord> for Item {
    type Error = Box<dyn Error>;

    fn try_from(record: &KinesisEventRecord) -> Result<Self, Self::Error> {
        let item = serde_json::from_slice::<Self>(&record.kinesis.data.0)?;
        Ok(item)
    }
}

/// Lambda handler.
fn my_handler(event: KinesisEvent, _ctx: Context) -> Result<(), HandlerError> {
    info!("Received event:  {:?}", event);
    match event.records.first() {
        None => {
            warn!("No Kinesis records present.");
            Ok(())
        },
        Some(record) => {
            let item = match Item::try_from(record) {
                Ok(i) => i,
                Err(e) => return Err(HandlerError::from(e.description())),
            };
            info!("Received the following Item object:  {:?}", item);
            Ok(())
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default())?;

    lambda!(my_handler);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::Item;
    use std::convert::TryFrom;
    use aws_lambda_events::event::kinesis::KinesisEvent;

    #[test]
    fn item_deserialization() {
        let kinesis_event_raw = include_str!("../test/kinesis_event.json");
        let kinesis_event = serde_json::from_str::<KinesisEvent>(kinesis_event_raw).unwrap();
        let record = kinesis_event.records.first().unwrap();
        println!("Record:  {:?}", record);
        let item = Item::try_from(record).unwrap();

        assert_eq!(1, item.id);
        assert_eq!("blah", item.description);
        assert_eq!(0, item.count);
    }
}
