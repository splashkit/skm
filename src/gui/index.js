const help = require('../commands/help')

// const table = new Table();

const execute = function (argv, callback) {
  // gui not implemented
  help.execute(argv, callback)
}

module.exports = {
  execute: execute
}
