//require simple-git with optional empty working path.
const git = require('simple-git')()
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')

const preExecuteOnCLI = function () {
  //read from CLI
  return []
}

const execute = function(args, callback) {
  if (utils.isMacOS) {
    const path = config['splashkit_install_location']
    const repo = config['splashkit_repo']

    logger.info("Mac Install command was executed. Cloning ")

    utils.doespathExist(`${path}/splashkit`)
    git.clone(repo, installPath)
    logger.info(`The repo was cloned to ${path} from ${repo}!`)
  }
}

module.exports = {
  execute: execute,
  preExecuteOnCLI: preExecuteOnCLI
}
