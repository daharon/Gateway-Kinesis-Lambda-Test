use std::convert::TryFrom;
use std::error::Error;
use log::{debug, error};

use aws_lambda_events::event::kinesis::KinesisEvent;
use cdrs::query::*;
use cdrs::query_values;
use lambda_runtime::Context;
use lambda_runtime::error::HandlerError;

use crate::cassandra;
use crate::cassandra::session::{CassandraSession, CASSANDRA_KEYSPACE, CASSANDRA_TABLE};
use crate::models::env::Config;
use crate::models::item::Item;


/// Handler for the AWS Lambda request.
/// Implements `lambda_runtime::Handler` for passing to the
/// `lambda_runtime::lambda!` macro.
pub struct GatewayKinesisLambdaTestHandler {
    cdb: CassandraSession,
    prepared_insert: PreparedQuery,
}

impl GatewayKinesisLambdaTestHandler {
    pub fn new(config: Config) -> Self {
        let cdb: CassandraSession = cassandra::session::new(&config.cassandra_host, config.cassandra_port);
        let insert_query = format!(r#"
            INSERT INTO {}.{}
            (id, description, count)
            VALUES (?, ?, ?)"#,
            CASSANDRA_KEYSPACE, CASSANDRA_TABLE);
        let prepared_insert = cdb.prepare(insert_query).unwrap();
        Self {
            cdb,
            prepared_insert
        }
    }
}

impl lambda_runtime::Handler<KinesisEvent, (), HandlerError> for GatewayKinesisLambdaTestHandler {

    /// This function is run on triggering events.
    fn run(&mut self, event: KinesisEvent, _ctx: Context) -> Result<(), HandlerError> {
        debug!("Received event:  {:?}", event);
        debug!("Number of received records:  {}", event.records.len());

        for record in event.records {
            let item = match Item::try_from(&record) {
                Ok(i) => i,
                Err(e) => return Err(HandlerError::from(e.description())),
            };
            debug!("De-serialized the following Item object:  {:?}", item);
            let values = query_values!(item.id, item.description, item.count);
            let result = self.cdb.exec_with_values(&self.prepared_insert, values);
            // TODO:  Perform bulk insert.
            if let Err(e) = result {
                error!("Failed to insert item with id {}", item.id);
                return Err(HandlerError::from(e.description()));
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::GatewayKinesisLambdaTestHandler;
    use aws_lambda_events::event::kinesis::KinesisEvent;
    use lambda_runtime::{Handler, Context};
    use crate::models::env::Config;

    #[test]
    fn instantiate_gateway_kinesis_lambda_test_handler() {
        let config = envy::from_env::<Config>().unwrap();
        let handler = GatewayKinesisLambdaTestHandler::new(config);
        // TODO:  Verify that keyspace and table are present.
    }

    #[test]
    fn process_kinesis_event() {
        let config = envy::from_env::<Config>().unwrap();
        let mut handler = GatewayKinesisLambdaTestHandler::new(config);
        let event_raw = include_str!("../test/kinesis_event.json");
        let event = serde_json::from_str::<KinesisEvent>(event_raw).unwrap();
        let ctx = Context {
            memory_limit_in_mb: 128,
            function_name: "test".to_string(),
            function_version: "".to_string(),
            invoked_function_arn: "".to_string(),
            aws_request_id: "".to_string(),
            xray_trace_id: None,
            log_stream_name: "".to_string(),
            log_group_name: "".to_string(),
            client_context: None,
            identity: None,
            deadline: 30
        };

        handler.run(event, ctx);

        // TODO: Verify that data is present in the Cassandra database.
    }
}
