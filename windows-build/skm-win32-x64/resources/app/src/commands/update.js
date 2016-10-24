const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')
const os = require('os')

const spinner = utils.getSpinner
spinner.text = 'Updating SplashKit! '

const installPath = `${os.homedir()}/${config['splashkit_install_name']}` // ~/.splashkit

const updateMac = function (callback) {
  const skmFolder = 'skm/mac-build'
  const installFolder = 'splashkit-macos'
  logger.debug('Update command was executed. Cloning repo')

  if (!utils.doespathExist(installPath)) {
    callback(Error(`can't find SplashKit, please install splashkit before updating!`))
  } else {
    spinner.start()
    utils.runGit(`git -C ${installPath}/skm pull &&
                  git -C ${installPath}/${installFolder} pull &&
                  unzip -o ${installPath}/${skmFolder}/skm.zip -d ${installPath}/${skmFolder} > ${installPath}/install.log &&
                  ln -sf ${installPath}/${skmFolder}/skm.app/Contents/MacOS/skm /usr/local/bin`, function (error, stdout, stderr) {
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
    updateMac(callback)
  } else if (utils.isLinux) {
    // installFolder = 'splashkit-linux'
    // skmFolder = 'skm/linux-build'
  } else if (utils.isWindows) {
    // installFolder = 'splashkit-windows'
    // skmFolder = 'skm/windows-build'
  }
}

module.exports = {
  execute: execute
}
