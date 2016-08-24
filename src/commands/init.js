//investigate better path solution
const utils = require('../utilities');
var fs = require('fs');
const logger = require('winston-color');
const mkdirp = require('mkdirp');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const execute = function(args, callback) {
    if (utils.isMacOS) {
      //check if we have the language
      if ( args == null || args.length == 0) {
        return logger.error ("No Arguments Supplied")
      }

      //check if this is already a SK folder
      if (utils.isSplashkit(`./`)) {
        return logger.error("can't init in an existing splashkit folder")
      }

      //generate a splashkitMeta object
      let dotSplashKit = utils.generateDotSplashkit();

      //check arguments to add language to splashkitMeta object
      dotSplashKit.language = utils.getValidLanguageFromArg(args[0])

      if (dotSplashKit.language != null)
        utils.writeDotSplashkit(dotSplashKit)
    }
    return
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}