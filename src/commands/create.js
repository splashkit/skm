const utils = require('../utils')
const fs = require('fs')
const init = require('./init')
const logger = require('winston-color')
const mkdirp = require('mkdirp')
const inquirer = require('inquirer')
const config = require('../config')

const _makeDirectory = function (path) {
  mkdirp.sync(`${path}/src`)
  mkdirp.sync(`${path}/images`)
  mkdirp.sync(`${path}/sounds`)
}

const _isDotFile = function(path) {
  return utils.isSplashKitDirectory(path)
}

const _createSplashKitProject = function (path, callback) {
  const splashKitData = utils.readDotSplashKit(path)
  if (splashKitData == null) {
    callback(Error(`Can't create splashKit Project`))
  }
  else if (splashKitData.status != 'initialized') {
    callback(Error(`Can't create SplashKit in a ${splashKitData.status} splashkit folder.`))
  }
  else {
    splashKitData.status = "created"
    _makeDirectory(path)
    utils.writeDotSplashKit(path, splashKitData)

    logger.info(`Created: ${splashKitData.language} SplashKit project: ${path} successfully.`)
  }
}

const preExecuteOnCLI = function(argv, callback) {
  const lang = argv['l'] || argv['language']
  const name = argv['n'] || argv['name']
  const isDotFile = _isDotFile('.')

  let willAskForName = null

  // don't need lang or name
  if (isDotFile) {
    callback(null, argv)
  }
  //valid name and lang
  else if (utils.isSupportedLangauge(lang)) {
    callback(null, argv)
  }
  else {
    let questions = [
      {
        type: 'confirm',
        name: 'want_named_project',
        message: 'Would you like to name your project? This will create a new folder.',
        default: true
      }
    ]

    inquirer.prompt(questions).then(function (answers) {
        let questions_2 = null

        if (answers['want_named_project']) {
          logger.info(`Do we want project named? ${answers['want_named_project']}`)
          questions_2 = [
            {
              type: 'input',
              name: 'project_name',
              message: 'What would you like to name your project?'
            },
            {
              type: 'list',
              name: 'language',
              message: `Which language would you like to initialise in this folder?`,
              choices: config['supported_languages']
            }
          ]
        }
        else {
          questions_2 = [
            {
              type: 'list',
              name: 'language',
              message: `Which language would you like to initialise in this folder?`,
              choices: config['supported_languages']
            }
          ]
        }
        logger.info(`inquiring second time:`)
        inquirer.prompt(questions_2).then(function (answers) {
          logger.info(`in body of second inquire`)
          argv['l'] = answers['language']
          argv['language'] = answers['language']

          if (answers['project_name'] != null) {
            argv['n'] = answers['project_name']
            argv['name'] = answers['project_name']
          }
          callback(null, argv)
        })
    })
  }
}

const execute = function(argv, callback) {
  const lang = argv['l'] || argv['language']
  const name = argv['n'] || argv['name']
  const workingFolder = name ==  null ? '.' : `./${name}`

  //check if we need to init or not and init if we need to.
  const isDotFile = _isDotFile('.')
  logger.debug(`Do we have dotfile?: ${isDotFile}`)

  if (!isDotFile && lang == null) {
    return callback(Error(`No language supplied. Use --language or -l to specify one.`))
  }
  else if (!isDotFile && !utils.isSupportedLangauge(lang)) {
    return callback(Error(`${lang} is unsupported. See help for supported languages.`))
  }
  else if (isDotFile && name) {
    return callback(Error(`Can't create SplashKit project in a existing project directory`))
  }
  else if (!isDotFile && utils.isSupportedLangauge(lang)) {
    logger.debug(`initing directory with language ${lang} at ${workingFolder}`)
    mkdirp.sync(workingFolder)
    utils.writeDotSplashKit(workingFolder, utils.generateDotSplashKitData(lang))
    logger.info(`Created: ${lang} SplashKit project: ${workingFolder} successfully.`)
  }
  _createSplashKitProject(workingFolder, callback)
  callback()
}

module.exports = {
    execute: execute,
    preExecuteOnCLI: preExecuteOnCLI
}