//require simple-git with optional empty working path.
const git = require("nodegit")
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')

const execute = function(args, callback) {
  if (utils.isMacOS) {
    const repo = config['splashkit_repo']
    const installPath = config['splashkit_install_location']

    logger.info("Mac Install command was executed. Cloning repo")

    if (utils.doespathExist(installPath)) {
      return callback(Error(`can't install, splashkit is already installed!`))
    }

    let cloneOptions = {}
    cloneOptions.fetchOpts = {
      callbacks: {
        certificateCheck: function() { return 1; }
      }
    };

    logger.info(`cloning ${repo} to ${installPath}`)
    let cloneRepo = git.Clone(repo, installPath, cloneOptions)
      .then(null, function(response){
        console.log(response)
        callback()
      })
  }
}

module.exports = {
  execute: execute
}
