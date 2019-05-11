# Testing API Gateway -> Kinesis -> Lambda

## Build

### JVM

### Rust

Use the [AWS Lambda Rust Docker Builder](https://github.com/softprops/lambda-rust) 
to cross-compile for the AWS Lambda environment.
```
$ docker run \
    --rm \
    --volume ${PWD}:/code \
    --volume ${HOME}/.cargo/registry:/root/.cargo/registry \
    --volume ${HOME}/.cargo/git:/root/.cargo/git \
    softprops/lambda-rust
```
The resulting ZIP file will be located in the `target/lambda/release`
directory.

## Deploy

```
$ aws cloudformation package \
    --template-file ./cloudformation.yaml \
    --output-template-file ./cloudformation.pkg.yaml \
    --force-upload \
    --s3-bucket us-aharon-test 
    
$ aws cloudformation deploy \
    --template-file ./cloudformation.pkg.yaml \
    --capabilities CAPABILITY_IAM \
    --stack-name 'api-kinesis-test-6' 
```
