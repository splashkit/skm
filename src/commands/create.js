const utils = require('../utilities');
var fs = require('fs');
var init = require('./init');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

function isSplashkit() {
  try {
    return fs.statSync('./.splashkit').isFile();
  } catch (e) {
    if (e.code === 'ENOENT') {
      return false;
    } else {
      throw e;
    }
  }
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

  console.log("in exec of create, args are: " + args)

  console.log("Is there a splashkit file?: " + isSplashkit())

    // readSplashkitFile(args, function(data) {
    //   console.log("data is: " + data)
    //   const splashKitData = JSON.parse(data)
    //   if (splashKitData.status == 'initialised')
    //   {
    //     console.log("initialised folder found, creating: " + splashKitData.language + ". Creating CPP Folder structure.")
    //   }
    //   else {
    //     console.log(splashKitData.status)
    //   }
    // })

    return

}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}