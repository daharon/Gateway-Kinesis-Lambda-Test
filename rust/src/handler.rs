use std::convert::TryFrom;
use log::{debug, info};

use aws_lambda_events::event::kinesis::KinesisEvent;
use lambda_runtime::Context;
use lambda_runtime::error::HandlerError;

use crate::models::item::Item;


/// Handler for the AWS Lambda request.
/// Implements `lambda_runtime::Handler` for passing to the
/// `lambda_runtime::lambda!` macro.
pub struct GatewayKinesisLambdaTestHandler {
    persistent_connection_test: bool,
}

impl lambda_runtime::Handler<KinesisEvent, (), HandlerError> for GatewayKinesisLambdaTestHandler {

    /// This function is run on triggering events.
    fn run(&mut self, event: KinesisEvent, _ctx: Context) -> Result<(), HandlerError> {
        debug!("Received event:  {:?}", event);
        self.toggle_persistent();

        for record in event.records {
            let item = match Item::try_from(&record) {
                Ok(i) => i,
                Err(e) => return Err(HandlerError::from(e.description())),
            };
            info!("Received the following Item object:  {:?}", item);
        }
        Ok(())
    }
}

impl GatewayKinesisLambdaTestHandler {
    pub fn new() -> Self {
        Self { persistent_connection_test: false }
    }

    /// Testing object persistence across Lambda invocations.
    fn toggle_persistent(&mut self) {
        info!("Persistent member value:  {}", self.persistent_connection_test);
        self.persistent_connection_test = !self.persistent_connection_test;
    }
}

