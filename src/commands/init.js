//investigate better path solution
const utils = require('../utils')
var fs = require('fs')
const logger = require('winston-color')
const mkdirp = require('mkdirp')
const config = require('../config')
var inquirer = require('inquirer');

const preExecuteOnCLI = function(argv, callback) {

    let questions = [
    {
      type: 'confirm',
      name: 'toBeDelivered',
      message: 'Is this for delivery?',
      default: false
    }
    ]

    let lang = argv['l']

    if (lang == null || !utils.isSupportedLangauge(lang)) {

      inquirer.prompt(questions).then(function (answers) {
          logger.info(`HERE2`)
          argv['l'] = answers
          callback(null, argv)
      });
    }
}

const execute = function(argv, callback) {
    //check if this is already a SK folder
    if (utils.isSplashKitDirectory('.')) {
      return callback(Error("Can't initialise in an existing SplashKit directory"))
    }

    //check if we have the language
    let lang = argv['l']
    if (lang == null) {
      return callback(Error(`${lang} language. See help for more details.`))
    }
    if (!utils.isSupportedLangauge(lang)) {
      return callback(Error(`${lang} is not a supported language. See help for more details.`))
    }

    utils.writeDotSplashKit('.', utils.generateDotSplashKitData(lang))
    callback()
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}
