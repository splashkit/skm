//require simple-git with optional empty working path.
const git = require('simple-git')();
const fs = require('fs');
const utils = require('../utilities');

const preExecuteOnCLI = function () {
  //read from CLI
  return [];
}

const execute = function(args, callback) {
  if (utils.isMacOS) {
    console.log("Mac Install command was executed. Cloning ");
    //user/home/.splashkit folder.
    git.clone("https://github.com/splashkit/splashkit", "./.splashkit-download");
    console.log("The file was saved!");
  }

  return
}


module.exports = {
  execute: execute,
  preExecuteOnCLI: preExecuteOnCLI
}
