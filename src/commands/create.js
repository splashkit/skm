const utils = require('../utils');
const fs = require('fs');
const init = require('./init');
const logger = require('winston-color');
const mkdirp = require('mkdirp');

const preExecuteOnCLI = function() {
    //read from CLI
    return [];
}

const _makeDirectory = function (path) {
  logger.log(`path is _makeDirector is: ${path}`)
  mkdirp.sync(`${path}/src`)
  mkdirp.sync(`${path}/images`)
  mkdirp.sync(`${path}/sounds`)
}

const _createSplashKitProject = function (path, callback) {
  const splashKitData = utils.readDotSplashKit(path)

  if (splashKitData == null) {
    return callback(Error(`Can't create splashKit Project `))
    // TODO: Need more specificity
  }

  if (splashKitData.status != 'initialized') {
    return callback(Error(`Can't create SplashKit in a ${splashKitData.status} splashkit folder.`))
  }

  splashKitData.status = "created"
  _makeDirectory(path)
  utils.writeDotSplashKit(path, splashKitData)

  logger.info(`Created: ${splashKitData.language} SplashKit project: ${path} successfully.`)
}

const execute = function(argv, callback) {
  const lang = argv['l'] || argv['language']
  const name = argv['n'] || argv['name']
  const workingFolder = name ==  null ? '.' : `./${name}`

  //check if we need to init or not and init if we need to.
  let isDotFile = utils.isSplashKitDirectory('.')
  logger.debug(`Do we have dotfile?: ${isDotFile}`)

  if (!isDotFile && lang == null) {
    return callback(Error(`No language supplied. Use --language or -l to specify one.`))
  }

  if (!isDotFile && !utils.isSupportedLangauge(lang)) {
    return callback(Error(`${lang} is unsupported. See help for supported languages.`))
  }

  if (isDotFile && name) {
    return callback(Error(`Can't create SplashKit project in a existing project directory`))
  }

  if (!isDotFile && utils.isSupportedLangauge(lang)) {
    logger.info(`initing directory with language ${lang}`)
    logger.debug(`working folder is: ${workingFolder}`)
    mkdirp.sync(workingFolder)
    utils.writeDotSplashKit(workingFolder, utils.generateDotSplashKitData(lang))
    logger.info(`Created: ${lang} SplashKit project: ${workingFolder} successfully.`)
  }

  _createSplashKitProject(workingFolder, callback)
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}