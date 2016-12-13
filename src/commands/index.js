const fs = require('fs')

let commandDir = __dirname

let cmds = fs.readdirSync(commandDir)
             .filter(cmd => ['index.js', 'compilers'].indexOf(cmd) === -1)
             .map(cmd => [cmd.slice(0, -3), require(commandDir + '/' + cmd)])

module.exports = new Map(cmds)
