//investigate better path solution
const utils = require('../utilities');
var fs = require('fs');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const execute = function(args, callback) {
    if (utils.isMacOS) {
      let data = utils.generateDotSplashkit();

      console.log(args)
      if (args) {
      switch (args[0].toLowerCase()) {
        case "cpp":
        case "c":
          data["language"] = "cpp";
          break;
        case "pascal":
        case "pas":
          data["language"] = "pascal";
          break;
        case "swift":
          data["language"] = "swift";
          break;
        case "c#":
        case "csharp":
          data["language"] = "C#";
          break;
      }
    }
    else {
      //no args
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