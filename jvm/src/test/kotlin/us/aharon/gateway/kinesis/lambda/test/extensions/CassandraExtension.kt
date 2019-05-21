package us.aharon.gateway.kinesis.lambda.test.extensions

import org.cassandraunit.utils.EmbeddedCassandraServerHelper
import org.junit.jupiter.api.extension.AfterAllCallback
import org.junit.jupiter.api.extension.BeforeAllCallback
import org.junit.jupiter.api.extension.ExtensionContext
import org.koin.test.KoinTest


class CassandraExtension : KoinTest, BeforeAllCallback, AfterAllCallback {

    override fun beforeAll(context: ExtensionContext) =
            EmbeddedCassandraServerHelper.startEmbeddedCassandra()

    override fun afterAll(context: ExtensionContext) =
            EmbeddedCassandraServerHelper.cleanEmbeddedCassandra()
}
