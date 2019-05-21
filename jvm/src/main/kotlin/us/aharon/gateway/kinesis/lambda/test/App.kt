package us.aharon.gateway.kinesis.lambda.test

import com.amazonaws.services.lambda.runtime.Context
import com.amazonaws.services.lambda.runtime.events.KinesisEvent
import org.koin.log.Logger.SLF4JLogger
import org.koin.standalone.StandAloneContext.startKoin

import us.aharon.gateway.kinesis.lambda.test.di.modules


class App {

    private val cassandraHandler = CassandraHandler()

    init {
        startKoin(listOf(modules), logger = SLF4JLogger())
    }

    /**
     * AWS Lambda handler.
     */
    fun handler(event: KinesisEvent, context: Context) =
            cassandraHandler.handler(event, context)
}
