const utils = require('../utils')
const logger = require('winston-color')
const inquirer = require('inquirer')
const config = require('../config')
const path = require(`path`)

let questions = []
const whatLang = {
  type: 'list',
  name: 'project_language',
  message: `Which language would you like to initialise in this folder?`,
  choices: config['supported_languages']
}
const whatName = {
  type: 'input',
  name: 'project_name',
  message: 'What would you like to name your project?'
}

const _createSplashKitProject = function (path, callback) {
  // COPY DIRECTORY HERE
  logger.debug('Here we will copy the directory')
  callback()
}

const preExecuteOnCLI = function (argv, callback) {
  let language = argv['l'] || argv['language']
  let name = argv['n'] || argv['name']

  // don't need lang or name
  if (utils.isSupportedLangauge(language) && name != null) {
    callback(null, argv)
  }
  // just ask for lang
  if (!utils.isSupportedLangauge(language)) {
    questions.push(whatLang)
  }
  // just ask for name
  if (name == null) {
    questions.push(whatName)
  }

  inquirer.prompt(questions).then(function (answers) {
    if (answers.project_language != null) {
      logger.debug(answers.project_language)
      language = answers['project_language']
    }
    if (answers.project_name != null) {
      name = answers.project_name
    }

    argv['l'] = argv['language'] = language
    argv['n'] = argv['name'] = name

    callback(null, argv)
  })
}

const execute = function (argv, callback) {
  const lang = argv['l'] || argv['language']
  const name = argv['n'] || argv['name']
  const workingFolder = name == null ? '.' : `./${name}`

  if (lang == null) {
    return callback(Error('No language supplied. Use --language or -l to specify one.'))
  } else if (!utils.isSupportedLangauge(lang)) {
    return callback(Error(`${lang} is unsupported. See help for supported languages.`))
  } else if (name == null) {
    return callback('Need name to create SplashKit project')
  }

  _createSplashKitProject(workingFolder, function (err, data) {
    if (err) {
      callback(err.message)
    } else {
      callback(null, `Successfully created ${lang} splashkit project in ${path.resolve(workingFolder)}`)
    }
  })
}

module.exports = {
  execute: execute,
  preExecuteOnCLI: preExecuteOnCLI
}
