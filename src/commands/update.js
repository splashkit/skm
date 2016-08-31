//require simple-git with optional empty working path.
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')
const cliSpinners = require('cli-spinners')
const ora = require('ora')
const os = require('os')

var repository;

const spinner = utils.getSpinner
spinner.text = 'Updating SplashKit! '


const execute = function(args, callback) {

  const repo = config['splashkit_repo']
  const installPath = `${os.homedir()}/splashkitTest`

  logger.info("Update command was executed. Cloning repo")

  if (!utils.doespathExist(installPath)) {
    callback(Error(`can't find SplashKit, please install splashkit before updating!`))
  } else {

    var repoDir = installPath

    spinner.start()

    utils.runGit(`git -C ~/splashkitTest/skm pull`, function(error, stdout, stderr) {
      if (error) {
        spinner.fail()
        return callback(error)
      } else {
        logger.info(stdout)
        utils.runGit(`git -C ~/splashkitTest/splashkit-macos pull`, function(error, stdout, stderr) {
          if (error) {
            spinner.fail()
            return callback(error)
          } else {
            logger.info(stdout)
            spinner.succeed()
            callback()
          }
        })
      }
    })
  }
}

module.exports = {
  execute: execute
}