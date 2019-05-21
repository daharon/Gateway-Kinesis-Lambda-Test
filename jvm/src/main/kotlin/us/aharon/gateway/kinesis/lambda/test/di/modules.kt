package us.aharon.gateway.kinesis.lambda.test.di

import com.datastax.driver.core.Cluster
import com.datastax.driver.core.Session
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import mu.KLogger
import mu.KotlinLogging
import org.koin.dsl.module.module

import us.aharon.gateway.kinesis.lambda.test.models.createKeyspaceAndTable


val modules = module {
    factory<KLogger> { (name: String) -> KotlinLogging.logger(name) }
    single<ObjectMapper> { ObjectMapper().registerKotlinModule() }
    single<Session> { setupCassandra() }
}

/**
 * Create the Cassandra connection session.
 * Immediately after connection, creates a keyspace and table.
 */
private fun setupCassandra(): Session {
    val session = Cluster.builder()
            .addContactPoint(System.getenv("CASSANDRA_HOST"))
            .withPort(System.getenv("CASSANDRA_PORT").toInt())
            .build()
            .connect()
    createKeyspaceAndTable(session)
    return session
}
