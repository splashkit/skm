const docs = require('../commands/docs')

const execute = function (argv, callback) {
  docs.execute(argv, callback)
}

module.exports = {
  execute: execute
}
