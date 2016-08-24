const utils = require('../utilities');
var fs = require('fs');
var init = require('./init');
var jsonminify = require("jsonminify");

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

// I Think there is a concurrency issue here.
// 3am update I don't think there is a concurrency issue at all...
function readSplashkitFile(args, callback) {
  if (utils.isMacOS) {
    fs.readFile('./.splashkit', 'utf8', function (err, data) {
      if (err) {
        return console.error("Can't read file");
      }
      callback(data)
    });
  }
}

const execute = function(args, callback) {
  //check if we need to init or not and init if we need to.
  if (!utils.isSplashkit()) {
    console.info("not found, initing directory with given language " + args[0])
    init.execute(args)
  }

  //read the .splashkit file
  readSplashkitFile(args, function(data) {

    const splashKitData = JSON.parse(JSON.minify(data))
    //now we have a init'd directory, so check its status

    if (args.length > 0 && splashKitData.language != args[0]) {
      return console.error(`can\'t create ${args[0]} in a ${splashKitData.language} splashkit folder.`)
    }

    if (splashKitData.status != 'initialised') {
      return console.error(`can\'t create Spalshkit in a ${splashKitData.status} splashkit folder.`)
    }

      console.info(`initialised folder found, creating: ${splashKitData.language} folder structure.`)
      splashKitData.status = "created"
      utils.writeDotSplashkit(splashKitData)
      // TODO: Create folder for correct langauge in splashKitData.language
  })
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}