const utils = require('../utilities');
const fs = require('fs');
const init = require('./init');
const winston = require('winston-color');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const execute = function(args, callback) {
  //check if we need to init or not and init if we need to.
  let isDotFile = utils.isSplashkit('./')

  if (!isDotFile && args.length > 0) {
    return winston.error(`no args given to an non splashkit folder splashkit folder.`)
  }
  if (!isDotFile)
  {
    winston.info(`initing directory with language ${args[0]}`)
    init.execute(args)
  }


  //read the .splashkit file
  const splashKitData = utils.readDotSplashkit()

  //now we have a init'd directory, so check its status
  if (args.length > 0 && splashKitData.language != args[0]) {
    return winston.error(`can\'t create ${args[0]} in a ${splashKitData.language} splashkit folder.`)
  }

  if (splashKitData.status != 'initialised') {
    return winston.error(`can\'t create Spalshkit in a ${splashKitData.status} splashkit folder.`)
  }

  winston.info(`initialised folder found, creating: ${splashKitData.language} folder structure.`)
  splashKitData.status = "created"
  utils.writeDotSplashkit(splashKitData)
  // TODO: Create folder for correct langauge in splashKitData.language

}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}