//require simple-git with optional empty working path.
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')
const cliSpinners = require('cli-spinners')
const ora = require('ora')
const os = require('os')

var repository;

const spinner = utils.getSpinner


const execute = function(args, callback) {

  const repo = config['splashkit_repo']
  const installPath = `${os.homedir()}/.splashkit/install-test`

  logger.info("Update command was executed. Cloning repo")

  if (!utils.doespathExist(installPath)) {
    callback(Error(`can't find SplashKit, please install splashkit before updating!`))
  } else {

    var repoDir = installPath

    spinner.text = 'Updating the SplashKit Manager! '
    spinner.start()

    utils.runGit(`git -C ~/splashkitTest/skm pull`, function(error, stdout, stderr) {
      if (error) {
        spinner.fail()
        return callback(error)
      } else {
        logger.info(stdout)
        spinner.text = 'Updating the SplashKit Mac Libraries! '
        utils.runGit(`git -C ~/splashkitTest/mac-os pull`, function(error, stdout, stderr) {
          logger.info(stdout)
          spinner.succeed()
          callback()
        })
      }
    })
  }
}

module.exports = {
  execute: execute
}