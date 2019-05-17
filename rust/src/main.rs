//! AWS Lambda function for testing
//! API Gateway -> Kinesis -> Lambda integration.

use std::error::Error;

use lambda_runtime::lambda;
use simplelog::SimpleLogger;

use gateway_kinesis_lambda_test::handler::GatewayKinesisLambdaTestHandler;
use gateway_kinesis_lambda_test::models::env::Config;


fn main() -> Result<(), Box<dyn Error>> {
    let config = envy::from_env::<Config>().unwrap();
    SimpleLogger::init(config.log_level, simplelog::Config::default())?;

    let handler = GatewayKinesisLambdaTestHandler::new(config);
    lambda!(handler);
    Ok(())
}

