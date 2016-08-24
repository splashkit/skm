const runCommand = require('../utilities').runCommand;

const execute = function(args, callback) {
  runCommand('./program')
}

module.exports = {
  execute: execute
}