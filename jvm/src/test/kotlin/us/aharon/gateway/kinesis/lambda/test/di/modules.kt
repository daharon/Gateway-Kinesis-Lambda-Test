package us.aharon.gateway.kinesis.lambda.test.di

import com.datastax.driver.core.Session
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import mu.KLogger
import mu.KotlinLogging
import org.cassandraunit.utils.EmbeddedCassandraServerHelper
import org.koin.dsl.module.module

import us.aharon.gateway.kinesis.lambda.test.models.createKeyspaceAndTable


val testModules = module {
    factory<KLogger> { (name: String) -> KotlinLogging.logger(name) }
    single<ObjectMapper> { ObjectMapper().registerKotlinModule() }
    single<Session> { setupCassandra() }
}

/**
 * Create the Cassandra connection session.
 * Immediately after connection, creates a keyspace and table.
 */
private fun setupCassandra(): Session {
    val session = EmbeddedCassandraServerHelper.getSession()
    createKeyspaceAndTable(session)
    return session
}
