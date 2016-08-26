//investigate better path solution
const utils = require('../utils')
const config = require('../config')
const inquirer = require('inquirer');

// Don't put this in utils, they need to be seperate there
const _checkLangIsValid = function (language) {
  return (language != null && utils.isSupportedLangauge(language))
}
//it is preExecutOnCLI's job to ensure argv is sanatized and a valid language
const preExecuteOnCLI = function(argv, callback) {
  const lang = argv['l'] || argv['language']
  //check the language, if it's fine continue on.
  if (_checkLangIsValid(lang)) {
    callback(null, argv)
  } else {
    let questions = [
      {
        type: 'list',
        name: 'language',
        message: `Which language would you like to initialise in this folder?`,
        choices: config['supported_languages']
      }
    ]
    inquirer.prompt(questions).then(function (answers) {
        argv['l'] = answers['language']
        callback(null, argv)
    });
  }
}

const execute = function(argv, callback) {
  const lang = argv['l'] || argv['language']
  //check if this is already a SK folder
  if (utils.isSplashKitDirectory('.')) {
    return callback(Error("Can't initialise in an existing SplashKit directory"))
  }
  else if (!_checkLangIsValid(lang)) {
    return callback(Error(`Error: Invalid language ${lang}`))
  }
  utils.writeDotSplashKit('.', utils.generateDotSplashKitData(lang))
  callback(null, '.')
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}
