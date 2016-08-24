//investigate better path solution
const utils = require('../utilities');
var fs = require('fs');
const winston = require('winston-color');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const execute = function(args, callback) {
    if (utils.isMacOS) {
      //check if we have the language
      if ( args == null || args.length == 0) {
        return winston.error ("No Arguments Supplied")
      }

      //check if this is already a SK folder
      if (utils.isSplashkit()) {
        return winston.error("can't init in an existing splashkit folder")
      }

      //generate a splashkitMeta object
      let dotSplashKit = utils.generateDotSplashkit();

      //check arguments to add language to splashkitMeta object
      switch (args[0].toLowerCase()) {
        case "cpp":
        case "c":
          dotSplashKit.language = "cpp";
          break;
        case "pascal":
        case "pas":
          dotSplashKit.language = "pascal";
          break;
        case "swift":
          dotSplashKit.language = "swift";
          break;
        case "c#":
        case "csharp":
          dotSplashKit.language = "C#";
          break;
        default:
          return winston.error (args[0].toLowerCase() + " is not a language.")
        }

        utils.writeDotSplashkit(dotSplashKit)
    }
    return
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}