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

const execute = function(args, callback) {

  console.log(`args are ${args}\n`)
  const lang = args[0]
  const workingFolder = (args[1] != null) ? args[1] : './SplashKit-Project'

  console.log(`project name is: ${workingFolder}\n`)

  //check if we need to init or not and init if we need to.
  let isDotFile = utils.isSplashKitDirectory('.')
  logger.info(`Do we have dotfile?: ${isDotFile}`)

  if (!isDotFile && !utils.isSupportedLangauge(lang)) {
    console.log(`lang is ${lang}\n`)
    return callback(Error(`no args given to an non splashkit folder.`))
  }

  if (isDotFile && lang != null) {
    return callback(Error(`can\'t create project with invalid language ${lang}`))
  }

  //no .file but we have a valid language

  logger.info(`Is dotfile?: ${isDotFile} and is language suppoted?: ${utils.isSupportedLangauge(lang)}`)

  if (!isDotFile && utils.isSupportedLangauge(lang)){
    logger.info(`initing directory with language ${lang}`)

    _makeDirectory(workingFolder)
    utils.writeDotSplashKit(workingFolder, utils.generateDotSplashKitData(lang))

    logger.info(`Created: ${lang} SplashKit project: ${workingFolder} successfully.`)
  }

  //read the .splashkit file
  const splashKitData = utils.readDotSplashKit(workingFolder)

  if (splashKitData.status != 'initialized') {
    return callback(Error(`can\'t create Spalshkit in a ${splashKitData.status} splashkit folder.`))
  }

  logger.info(`Creating: ${splashKitData.language} SplashKit project.`)
  splashKitData.status = "created"

  _makeDirectory(workingFolder)
  utils.writeDotSplashKit(workingFolder, splashKitData)

  logger.info(`Created: ${splashKitData.language} SplashKit project: ${workingFolder} successfully.`)
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}