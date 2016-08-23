//investigate better path solution
const utils = require('../utilities');
var fs = require('fs');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const execute = function(args, callback) {
    if (utils.isMacOS) {

      fs.writeFile("./.splashkit", utils.createSKJSON(), function(err) {
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