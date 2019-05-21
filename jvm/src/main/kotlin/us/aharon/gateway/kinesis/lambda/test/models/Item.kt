package us.aharon.gateway.kinesis.lambda.test.models


data class Item(
        val id: Int,
        val description: String,
        val count: Int
)
