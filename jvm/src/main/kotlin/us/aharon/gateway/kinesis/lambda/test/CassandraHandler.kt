package us.aharon.gateway.kinesis.lambda.test

import com.amazonaws.services.lambda.runtime.Context
import com.amazonaws.services.lambda.runtime.events.KinesisEvent
import com.datastax.driver.core.Session
import com.datastax.driver.core.querybuilder.QueryBuilder
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import mu.KLogger
import org.koin.core.parameter.parametersOf
import org.koin.standalone.KoinComponent
import org.koin.standalone.inject

import us.aharon.gateway.kinesis.lambda.test.models.KEYSPACE
import us.aharon.gateway.kinesis.lambda.test.models.TEST_TABLE
import us.aharon.gateway.kinesis.lambda.test.models.Item

import java.util.Base64


/**
 * Receives Kinesis events and write to Cassandra.
 */
class CassandraHandler : KoinComponent {

    private val log: KLogger by inject { parametersOf(this::class.java.simpleName) }
    private val cdb: Session by inject()
    private val json: ObjectMapper by inject()


    fun handler(event: KinesisEvent, context: Context) {
        log.debug { "Received the following event: $event" }
        log.debug { "Number of received records:  ${event.records.size}" }

        event.records.forEach {
            val dataBytes = Base64.getEncoder().encode(it.kinesis.data.array())
            val dataString = Base64.getDecoder().decode(dataBytes)
            val item = json.readValue<Item>(dataString)
            log.debug { "De-serialized the following Item object:  $item" }
            val insertItem = QueryBuilder.insertInto(KEYSPACE, TEST_TABLE).using()
                    .value("id", item.id)
                    .value("description", item.description)
                    .value("count", item.count)
            cdb.execute(insertItem)
        }
    }
}
