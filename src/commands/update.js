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
  const installName = config['splashkit_install_name']
  const installPath = `${os.homedir()}/${installName}`

  logger.info("Update command was executed. Cloning repo")

  if (!utils.doespathExist(installPath)) {
    callback(Error(`can't find SplashKit, please install splashkit before updating!`))
  } else {

    spinner.start()

    utils.runGit(`git -C ${installPath}/skm pull`, function(error, stdout, stderr) {
      if (error) {
        spinner.fail()
        return callback(error)
      } else {
        logger.info(stdout)
        utils.runGit(`git -C ${installPath}/splashkit-macos pull`, function(error, stdout, stderr) {
          if (error) {
            spinner.fail()
            return callback(error)
          } else {
            logger.info(stdout)
            logger.info('updated!')
            utils.runCommand(`unzip -o ${installPath}/skm/mac-build/skm.zip -d ~/.splashkit/skm/mac-build > ${installPath}/install.log && ln -sf ${installPath}/skm/mac-build/skm.app/Contents/MacOS/skm /usr/local/bin`, function() {
              spinner.succeed()
              callback()
            })
          }
        })
      }
    })
  }
}

module.exports = {
  execute: execute
}