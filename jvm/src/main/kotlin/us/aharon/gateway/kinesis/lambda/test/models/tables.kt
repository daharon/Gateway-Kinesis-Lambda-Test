package us.aharon.gateway.kinesis.lambda.test.models

import com.datastax.driver.core.DataType
import com.datastax.driver.core.Session
import com.datastax.driver.core.schemabuilder.SchemaBuilder


const val KEYSPACE = "test_ks"
const val TEST_TABLE = "test_table"


/**
 * Create the Cassandra keyspace and table.
 */
internal fun createKeyspaceAndTable(session: Session) {
    val createKeyspace = SchemaBuilder.createKeyspace(KEYSPACE)
            .ifNotExists()
            .with()
            .replication(mapOf("class" to "SimpleStrategy", "replication_factor" to 1))
    session.execute(createKeyspace)
    val createTable = SchemaBuilder.createTable(KEYSPACE, TEST_TABLE)
            .ifNotExists()
            .addPartitionKey("id", DataType.cint())
            .addColumn("description", DataType.text())
            .addColumn("count", DataType.cint())
    session.execute(createTable)
}
