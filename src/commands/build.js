const runCommand = require('../utilities').runCommand;

const execute = function(args, callback) {
  runCommand('clang++ ./game_main.cpp -o program')
}

module.exports = {
  execute: execute
}