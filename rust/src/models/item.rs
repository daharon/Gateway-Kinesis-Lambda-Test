use std::convert::TryFrom;
use std::error::Error;

use aws_lambda_events::event::kinesis::KinesisEventRecord;
use serde::Deserialize;


/// The item sent by Kinesis.
/// Structure defined in `cloudformation.yaml` under the [ItemModel] resource.
#[derive(Deserialize, Debug)]
pub struct Item {
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


#[cfg(test)]
mod test {
    use super::Item;
    use std::convert::TryFrom;
    use aws_lambda_events::event::kinesis::KinesisEvent;

    #[test]
    fn item_deserialization() {
        let kinesis_event_raw = include_str!("../../test/kinesis_event.json");
        let kinesis_event = serde_json::from_str::<KinesisEvent>(kinesis_event_raw).unwrap();
        let record = kinesis_event.records.first().unwrap();
        println!("Record:  {:?}", record);
        let item = Item::try_from(record).unwrap();

        assert_eq!(1, item.id);
        assert_eq!("blah", item.description);
        assert_eq!(0, item.count);
    }
}
