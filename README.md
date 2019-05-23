# Testing API Gateway -> Kinesis -> Lambda

## Build

### JVM

```
$ cd jvm
$ mvn install
```
The resulting JAR file will be located in the `target` directory.

### Rust

Use the [AWS Lambda Rust Docker Builder](https://github.com/softprops/lambda-rust) 
to cross-compile for the AWS Lambda environment.
```
$ cd rust
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
    --force-upload \
    --parameter-overrides \
        "MyIP=$(curl --silent ipecho.net/plain)" \
        'VPC=vpc-foo123' \
        'Subnets=subnet-bar456,subnet-baz789' \
    --stack-name 'api-kinesis-test-6' 
```

## Pricing

<table>
    <tr>
        <td rowspan="2">API Gateway</td>
        <td>$3.50/M requests</td>
    </tr>
    <tr>
        <td>$0.09/GB</td>
    </tr>
    <tr>
        <td rowspan="3">Kinesis</td>
        <td>$0.015/shard/hour</td>
    </tr>   
    <tr>
        <td>$0.014/M writes</td>
    </tr>
    <tr>
        <td>$0.013/GB read</td>
    </tr>
</table>
