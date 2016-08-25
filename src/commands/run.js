const runCommand = require('../utils').runCommand;

const execute = function(args, callback) {
  runCommand('./program')
}

module.exports = {
  execute: execute
}