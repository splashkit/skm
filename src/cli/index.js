const commands = require('../commands')
const logger = require('winston-color')

const _executeCommand = function (cmd, argv, callback) {
  cmd.execute(argv, function(err, data) {
    if (err) {
      logger.error(`Error executing ${cmd.cmdName}:\n\t${err}`)
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
        callback(logger.error(`${cmdName} is not a valid command.`))
    } else {
      //add the command name string to the object for error checking later on.
      cmd.cmdName = cmdName

      if (typeof cmd.preExecuteOnCLI === "function") {
          cmd.preExecuteOnCLI(argv, function (error, argv) {
            if (error != null) {
              callback(error)
            } else {
              _executeCommand(cmd, argv, callback)
            }
          })
      } else {
        _executeCommand(cmd, argv, callback)
      }
    }
}

module.exports = {
    execute: execute
}
