const commands = require('../commands');

/**
 * Execute a given command name with the given args and callback.
 */
const execute = function(cmdName, args, callback) {
    let cmd = commands.get(cmdName);
    if (cmd === undefined) {
        console.error(`No such command name ${cmdName}`);
    } else {
        console.info(`Executing ${cmdName} command...`);
        // if (cmd.preExecuteOnCLI === function) {
        //     args += cmd.preExecuteOnCLI()
        // }
        cmd.execute(args, callback);
    }
}

module.exports = {
    execute: execute
}