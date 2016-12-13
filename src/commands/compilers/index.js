const logger = require('winston-color')

const _getExternalCommand = function (externalCommandName) {
  let externalCommands
  const home = process.env.HOME
  try {
    externalCommands = require(`${home}/.splashkit/commands`)
  } catch (error) {
    logger.debug(error)
    return // Return null if you can't find the directory.
  }
  return externalCommands.get(externalCommandName)
}

const hasExternalCommandNamed = function (externalCommandName) {
  return _getExternalCommand(externalCommandName) != null
}

const execute = function (argv, callback) {
  const externalCommandName = argv['_'][0]
  const command = _getExternalCommand(externalCommandName)
  // check for sigabort
  try {
    command.execute(argv, function (err, string) {
      if (err) {
        callback(err)
      } else {
        callback(null, string)
      }
    })
  } catch (error) {
    callback(error)
  }
}

module.exports = {
  execute: execute,
  hasExternalCommandNamed: hasExternalCommandNamed
}
