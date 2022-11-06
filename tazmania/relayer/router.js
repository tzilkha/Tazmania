const router = require('express').Router()
const { fee, relayer_id } = require('./config')
const { get_commitments } = require('./explorer')
const { withdraw } = require('./proof')

// Add CORS headers
router.use((req, res, next) => {
  res.header('X-Frame-Options', 'DENY')
  res.header('Access-Control-Allow-Origin', '*')
  res.header('Access-Control-Allow-Headers', 'Origin, X-Requested-With, Content-Type, Accept')
  next()
})

// Log error to console but don't send it to the client to avoid leaking data
router.use((err, req, res, next) => {
  if (err) {
    console.error(err)
    return res.sendStatus(500)
  }
  next()
})


function info(req, res) {
	res.json({
		'fee': fee,
		'relayer_id': relayer_id
  })
}

router.get('/info', info)
router.get('/commitments', get_commitments)
router.post('/withdraw', withdraw)


module.exports = router

