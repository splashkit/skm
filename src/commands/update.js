const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')

const spinner = utils.getSpinner
spinner.text = 'Updating SplashKit! '

const home = process.env.HOME
const installPath = `${home}/${config['splashkit_install_name']}` // ~/.splashkit

let installFolder

const updateSplashKit = function (callback) {
  logger.debug('Update command was executed. Updating SplashKit')

  if (!utils.doespathExist(installPath)) {
    callback(Error(`can't find SplashKit, please install splashkit before updating!`))
  } else {
    spinner.start()
    utils.runGit(`git -C ${installPath}/${installFolder} pull`, function (error, stdout, stderr) {
      if (error) {
        spinner.fail()
        return callback(error)
      } else {
        spinner.succeed()
        console.log(stdout)
        callback()
      }
    })
  }
}

const execute = function (args, callback) {
  if (utils.isMacOS) {
    installFolder = 'splashkit-macos'
  } else if (utils.isLinux) {
    installFolder = 'splashkit-windows'
  } else if (utils.isWindows) {
    installFolder = 'splashkit-linux'
  }
  updateSplashKit(callback)
}

module.exports = {
  execute: execute
}
