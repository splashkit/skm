//require simple-git with optional empty working path.
const git = require("nodegit")
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')
const os = require('os')

let spinner = utils.getSpinner
spinner.text = 'Installing Splashkit'

const cloneOptions = {
  fetchOpts: {
    callbacks: {
      certificateCheck: function() {
        return 1;
      }
    }
  }
}

const execute = function(args, callback) {
  const repo = config['splashkit_repo']
  const installPath = `${os.homedir()}/.splashkit/install-test`

  logger.info("Mac Install command was executed. Cloning repo")

  // not sure if I need this, as the clone command will check the path
  if (utils.doespathExist(installPath)) {
    callback(Error(`can't install at ${installPath}, splashkit is already installed!`))
  } else {
  spinner.start()

  git.Clone(repo, installPath, cloneOptions)
    .then(null, function(response) {
      if (response === true) {
        spinner.succeed()
        callback()
      } else {
        callback(response)
      }
    })
  }
}

module.exports = {
  execute: execute
}