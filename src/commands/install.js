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

  if (utils.doespathExist(installPath)) {
    callback(Error(`can't install at ${installPath}, splashkit is already installed!`))
  } else {
    spinner.start()

    utils.runGit(`git clone ${config.splashkit_repo} ${installPath}`, function () {
      spinner.succeed()
      callback()
    })
  }
}

module.exports = {
  execute: execute
}