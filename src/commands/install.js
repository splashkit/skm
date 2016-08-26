//require simple-git with optional empty working path.
const git = require("nodegit")
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')

const execute = function(args, callback) {
  if (utils.isMacOS) {
    logger.log("Before CLONE")

    const repo = config['splashkit_repo']
    const installPath = config['splashkit_install_location']

    logger.info("Mac Install command was executed. Cloning ")

    if (utils.doespathExist(installPath)) {
      //return callback(Error(`can't install, splashkit is already installed!`))
    }

    // Clone a given repository into the `./tmp` folder.
    logger.info(`cloning ${repo} to ${installPath}`)
    git.Clone(repo, installPath)
    logger.info(`cloned ${repo} to ${installPath}`)

  }
}

module.exports = {
  execute: execute
}
