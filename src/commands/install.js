//require simple-git with optional empty working path.
const git = require("nodegit")
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')
const cliSpinners = require('cli-spinners')
const ora = require('ora')
const os = require('os')

const spinner = new ora({
  text: 'Installing Splashkit',
  spinner: utils.randomJsonValue(cliSpinners),
  color: 'cyan'
});

const execute = function(args, callback) {
  if (utils.isMacOS) {
    const repo = config['splashkit_repo']
    const installPath = `${os.homedir()}/.splashkit`

    logger.info("Mac Install command was executed. Cloning repo")

    // not sure if I need this, as the clone command will check the path
    if (utils.isSplashKitDirectory(installPath)) {
      callback(Error(`can't install at ${installPath}, splashkit is already installed!`))
    } else {
      let cloneOptions = {}
      cloneOptions.fetchOpts = {
        callbacks: {
          certificateCheck: function() { return 1; }
        }
      };

      logger.info(`cloning ${repo} to ${installPath}`)
      let cloneRepo = git.Clone(repo, installPath, cloneOptions)
        .then(null, function(response){
          if (response === true) {
            spinner.succeed()
            callback()
          } else {
            callback(response)
          }
        })
        spinner.start(cliSpinners.dots4);
      }
  }
}

module.exports = {
  execute: execute
}
