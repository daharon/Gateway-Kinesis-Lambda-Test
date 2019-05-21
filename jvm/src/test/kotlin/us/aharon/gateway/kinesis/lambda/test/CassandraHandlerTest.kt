package us.aharon.gateway.kinesis.lambda.test

import com.amazonaws.services.lambda.runtime.*
import com.amazonaws.services.lambda.runtime.events.KinesisEvent
import com.datastax.driver.core.Session
import com.datastax.driver.core.querybuilder.QueryBuilder
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.extension.ExtendWith
import org.junit.jupiter.api.extension.Extensions
import org.koin.standalone.inject
import org.koin.test.KoinTest

import us.aharon.gateway.kinesis.lambda.test.models.KEYSPACE
import us.aharon.gateway.kinesis.lambda.test.models.TEST_TABLE
import us.aharon.gateway.kinesis.lambda.test.extensions.CassandraExtension
import us.aharon.gateway.kinesis.lambda.test.extensions.LoadModulesExtension

import java.nio.ByteBuffer
import java.util.Date
import kotlin.test.assertNotNull


@Extensions(
    ExtendWith(LoadModulesExtension::class),
    ExtendWith(CassandraExtension::class))
class CassandraHandlerTest : KoinTest {

    private val testContext = object : Context {
        override fun getAwsRequestId(): String = "FAKE_REQUEST_ID"
        override fun getLogStreamName(): String = "FAKE_LOG_STREAM_NAME"
        override fun getClientContext(): ClientContext = object : ClientContext {
            override fun getCustom(): MutableMap<String, String> = mutableMapOf()
            override fun getEnvironment(): MutableMap<String, String> = mutableMapOf()
            override fun getClient(): Client = object : Client {
                override fun getAppVersionCode(): String = "0.0.1"
                override fun getAppPackageName(): String = "FAKE"
                override fun getAppTitle(): String = "FAKE"
                override fun getInstallationId(): String = "FAKE"
                override fun getAppVersionName(): String = "FAKE"
            }
        }
        override fun getFunctionName(): String = ""
        override fun getRemainingTimeInMillis(): Int = 9_000_000
        override fun getLogger(): LambdaLogger = object : LambdaLogger {
            override fun log(p0: String?) = println(p0)
            override fun log(p0: ByteArray?) = println(p0)
        }
        override fun getInvokedFunctionArn(): String = "FAKE"
        override fun getMemoryLimitInMB(): Int = 3008
        override fun getLogGroupName(): String = "FAKE_LOG_GROUP_NAME"
        override fun getFunctionVersion(): String = "0.1.0"
        override fun getIdentity(): CognitoIdentity = object : CognitoIdentity {
            override fun getIdentityPoolId(): String = "FAKE"
            override fun getIdentityId(): String = "FAKE"
        }
    }
    private val singleKinesisEvent = KinesisEvent().apply {
        records = listOf(
                KinesisEvent.KinesisEventRecord().apply {
                    awsRegion = "us-east-1"
                    eventName = "aws:kinesis:record"
                    eventSourceARN = "arn:aws:kinesis:EXAMPLE"
                    eventSource = "aws:kinesis"
                    eventVersion = "1.0"
                    eventID = "shardId-000000000000:49545115243490985018280067714973144582180062593244200961"
                    invokeIdentityArn = "arn:aws:iam::EXAMPLE"
                    kinesis = KinesisEvent.Record()
                            .withApproximateArrivalTimestamp(Date(1428537600))
                            .withPartitionKey("partitionKey-03")
                            .withSequenceNumber("49545115243490985018280067714973144582180062593244200961")
                            .withData(ByteBuffer.wrap("{\"id\": 1, \"description\": \"blah\", \"count\": 0}".toByteArray()))
                            as KinesisEvent.Record
                }
        )
    }


    @Test
    fun `Cassandra session creation`() {
        val cdb: Session by inject()

        // Verify that the keyspace and table exist.
        val keyspaceQuery = QueryBuilder.select().all().from("system_schema", "keyspaces")
        val keyspaces = cdb.execute(keyspaceQuery)
        assertNotNull(keyspaces.find { it.getString("keyspace_name") == KEYSPACE })

        val tablesQuery = QueryBuilder.select().all().from("system_schema", "tables")
        val tables = cdb.execute(tablesQuery)
        assertNotNull(tables.find { it.getString("keyspace_name") == KEYSPACE && it.getString("table_name") == TEST_TABLE })
    }

    @Test
    fun `Single Kinesis event`() {
        CassandraHandler().handler(singleKinesisEvent, testContext)

        // Check to see if the `Item` is present.
        val cdb: Session by inject()
        val query = QueryBuilder.select().all().from(KEYSPACE, TEST_TABLE).limit(1)
        val results = cdb.execute(query)
        val insertedItem = results.first()
        assert(1 == insertedItem.getInt("id"))
        assert("blah" == insertedItem.getString("description"))
        assert(0 == insertedItem.getInt("count"))
    }
}
