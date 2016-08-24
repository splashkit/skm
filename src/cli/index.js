const commands = require('../commands');
const winston = require('winston-color');

/**
 * Execute a given command name with the given args and callback.
 */
const execute = function(cmdName, args, callback) {
    let cmd = commands.get(cmdName);
    if (cmd === undefined) {
        winston.error(`No such command name ${cmdName}`);
    } else {
        winston.info(`Executing ${cmdName} command...`);
        // if (cmd.preExecuteOnCLI === function) {
        //     args += cmd.preExecuteOnCLI()
        // }
        cmd.execute(args, callback);
    }
}

module.exports = {
    execute: execute
}