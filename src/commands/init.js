//investigate better path solution
const utils = require('../utilities');
var fs = require('fs');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const execute = function(args, callback) {
    if (utils.isMacOS) {
      if (utils.isSplashkit())
      {
        return console.error("can't init in an existing splashkit folder")
      }

      let data = utils.generateDotSplashkit();

      if ( args == null || args.length == 0) {
        return console.error ("No Arguments Supplied")
      }

      //check arguments
      switch (args[0].toLowerCase()) {
        case "cpp":
        case "c":
          data.language = "cpp";
          break;
        case "pascal":
        case "pas":
          data.language = "pascal";
          break;
        case "swift":
          data.language = "swift";
          break;
        case "c#":
        case "csharp":
          data.language = "C#";
          break;
        default:
          return console.error (args[0].toLowerCase() + " is not a language.")
        }

      fs.writeFile('./.splashkit', JSON.stringify(data, null, "\t"), (err) => {
        if (err) throw err;
        console.log('Saved to ./.splashkit successfully.');
      });
    }
    return
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}