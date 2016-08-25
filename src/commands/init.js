//investigate better path solution
const utils = require('../utils');
var fs = require('fs');
const logger = require('winston-color');
const mkdirp = require('mkdirp');
const config = require('../config');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const execute = function(args, callback) {
    //check if we have the language
    if ( args == null || args.length == 0) {
      return callback(Error("No arguments supplied"))
    }

    //check if this is already a SK folder
    if (utils.isSplashKitDirectory('.')) {
      return callback(Error("Can't initialise in an existing SplashKit directory"))
    }

    let lang = args[0]; // todo fix!
    if (!utils.isSupportedLangauge(lang)) {
      return callback(Error("Soz need lang to be cool. See help for more."))
    }

    const data = utils.generateDotSplashKitData(lang);
    utils.writeDotSplashKit('.', data);
    callback();
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}