const commands = require('../commands');
const logger = require('winston-color');

/**
 * Execute a given command name with the given args and callback.
 */
const execute = function(cmdName, args, callback) {
    let cmd = commands.get(cmdName);
    if (cmd === undefined) {
        logger.error(`No such command name ${cmdName}`);
    } else {
        logger.info(`Executing ${cmdName} command...`);
        // if (cmd.preExecuteOnCLI === function) {
        //     args += cmd.preExecuteOnCLI()
        // }
        cmd.execute(args, function(err, data) {
          if (err) {
            logger.error(`Error executing ${cmdName}:\n\t${err.message}`)
          } else if (data != null) {
            logger.info(data)
          }
        });
    }
}

module.exports = {
    execute: execute
}