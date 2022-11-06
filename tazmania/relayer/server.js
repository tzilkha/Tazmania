const express = require('express')
const router = require('./router')
const { port } = require('./config')

const app = express()
app.use(express.json())
app.use(router)
app.listen(port)
console.log(`Tazmania Relayer started on port ${port}.`)