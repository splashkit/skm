const {runCommand} = require('../utilities');

const execute = function(args, callback) {
  runCommand('clang++ ./game_main.cpp -o program', callback)
}

module.exports = {
  execute: execute
}