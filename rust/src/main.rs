//! AWS Lambda function for testing
//! API Gateway -> Kinesis -> Lambda integration.

use std::error::Error;
use log::LevelFilter;

use lambda_runtime::lambda;
use simplelog::SimpleLogger;

use gateway_kinesis_lambda_test::handler::GatewayKinesisLambdaTestHandler;


fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default())?;

    let handler = GatewayKinesisLambdaTestHandler::new();
    lambda!(handler);
    Ok(())
}

