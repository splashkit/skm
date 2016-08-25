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

const execute = function(argv, callback) {
    //check if this is already a SK folder
    if (utils.isSplashKitDirectory('.')) {
      return callback(Error("Can't initialise in an existing SplashKit directory"))
    }

    //check if we have the language
    let lang = argv['l'];
    if (lang == null) {
      return callback(Error(`${lang} language. See help for more details.`))
    }
    if (!utils.isSupportedLangauge(lang)) {
      return callback(Error(`${lang} is not a supported language. See help for more details.`))
    }

    utils.writeDotSplashKit('.', utils.generateDotSplashKitData(lang));
    callback();
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}