//require simple-git with optional empty working path.
const git = require('simple-git')()
const fs = require('fs')
const utils = require('../utils')
const logger = require('winston-color')

const preExecuteOnCLI = function () {
  //read from CLI
  return []
}

const execute = function(args, callback) {
  if (utils.isMacOS) {
    logger.info("Mac Install command was executed. Cloning ")
    //user/home/.splashkit folder.
    git.clone("https://github.com/splashkit/splashkit", "./.splashkit-download")
    logger.info("The repo was cloned!")
  }
  return
}

module.exports = {
  execute: execute,
  preExecuteOnCLI: preExecuteOnCLI
}
