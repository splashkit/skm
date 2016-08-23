const utils = require('../utilities');
var fs = require('fs');
var init = require('./init');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}



//why is there weird concurrency issue?
function readSplashkitFile(args, callback) {
  if (utils.isMacOS) {
    fs.readFile('./.splashkit', 'utf8', function (err, data) {
      if (err) {
        if (err.code == 'ENOENT') {
          init.execute(args)
        }
        else {
          console.log(err)
          return
        }
      }
      callback(data)
    });
  }
}

const execute = function(args, callback) {

  console.log("create exec args are: " + args)
  //check if we need to init or not and init if we need to.
  if (!utils.isSplashkit()) {
    console.log("not found, initing directory with given language " + args[0])
    init.execute(args)
  }

  readSplashkitFile(args, function(data) {
    const splashKitData = JSON.parse(data)
    //now we have a init'd directory, so check its status
    if (splashKitData.status == 'initialised')
    {
      console.log("initialised folder found, creating: " + splashKitData.language + " folder structure.")
      //TODO: Create folder for correct langauge in splashKitData.language
    }
    else {
      console.error('can\'t create Spalshkit in a ' + splashKitData.status + "splashkit folder.")
    }
  })

  return

}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}