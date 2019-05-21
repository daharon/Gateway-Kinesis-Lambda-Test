package us.aharon.gateway.kinesis.lambda.test.extensions

import org.junit.jupiter.api.extension.AfterAllCallback
import org.junit.jupiter.api.extension.BeforeAllCallback
import org.junit.jupiter.api.extension.ExtensionContext
import org.koin.log.PrintLogger
import org.koin.standalone.StandAloneContext
import org.koin.test.KoinTest

import us.aharon.gateway.kinesis.lambda.test.di.testModules


class LoadModulesExtension : KoinTest, BeforeAllCallback, AfterAllCallback {

    override fun beforeAll(context: ExtensionContext) {
        StandAloneContext.startKoin(listOf(testModules), logger = PrintLogger())
    }

    override fun afterAll(context: ExtensionContext) {
        StandAloneContext.stopKoin()
    }
}
