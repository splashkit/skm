const fs = require('fs')

let commandDir = __dirname
let cmds = fs.readdirSync(commandDir)
             .filter(cmd => cmd != 'index.js' )
             .map(cmd => [cmd.slice(0, -3), require(__dirname + '/' + cmd)] )

module.exports = new Map(cmds)
