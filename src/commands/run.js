const runCommand = require('../utils').runCommand

const execute = function(args, callback) {
  runCommand('./program')
  //callback()
}

module.exports = {
  execute: execute
}
