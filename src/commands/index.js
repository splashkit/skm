const fs = require('fs')
const path = require('path')

let commandDir = __dirname

// Test this map
let cmds = fs.readdirSync(commandDir)
             .filter(cmd => ['index.js', 'compilers'].indexOf(cmd) === -1)
             .map(cmd => [cmd.slice(0, -3), require(path.resolve(__dirname, '/', cmd))])

module.exports = new Map(cmds)
