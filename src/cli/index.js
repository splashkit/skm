const commands = require('../commands')
const logger = require('winston-color')

const _executeSpecificCommand = function (cmd, argv, callback) {
  cmd.execute(argv, function(err, data) {
    if (err) {
      logger.error(`Error executing ${cmdName}:\n\t${err}`)
    } else if (data != null) {
      logger.info(data)
    }
    callback()
  })
}

/**
 * Execute a given command name with the given args and callback.
 */
const execute = function(cmdName, argv, callback) {
    const cmd = commands.get(cmdName)
    if (cmd == null) {
        return logger.error(`No such command name ${cmdName}`)
    }
    if (typeof cmd.preExecuteOnCLI === "function") {
        cmd.preExecuteOnCLI(cmdName, function (error, argv) {
          _executeSpecificCommand(cmd, argv, callback)
        })
    } else {
      _executeSpecificCommand(cmd, argv, callback)
    }
}

module.exports = {
    execute: execute
}
