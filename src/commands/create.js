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
  console.log(`path is _makeDirector is: ${path}`)
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
  console.log(`args are ${argv}\n`)
  const lang = argv['l']
  const workingFolder = (argv['n'] == null) ? '.' : `./${argv['n']}`

  console.log(`project name is: ${workingFolder}\n`)

  //check if we need to init or not and init if we need to.
  let isDotFile = utils.isSplashKitDirectory('.')
  logger.info(`Do we have dotfile?: ${isDotFile}`)

  if (!isDotFile && !utils.isSupportedLangauge(lang)) {
    console.log(`lang is ${lang}\n`)
    return callback(Error(`${lang} is unsupported, or no language given to an non splashkit folder.`))
    //correct
  }

  if (!isDotFile && utils.isSupportedLangauge(lang)) {
    logger.info(`initing directory with language ${lang}`)
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