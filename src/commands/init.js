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

      fs.writeFile("./.splashkit", JSON.stringify(data, null, "\t"), function(err) {
          if (err) {
              return console.log(err);
          }
          console.log("The file was saved!");
      });
    }
    return
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}