const fs = require('fs')
const config = require('../config')
const childProcess = require('child_process')
const Ora = require('ora')
const cliSpinners = require('cli-spinners')

const randomJsonValue = function (obj) {
  const keys = Object.keys(obj)
  return obj[keys[keys.length * Math.random() << 0]]
}

const getSpinner = new Ora({
  spinner: randomJsonValue(cliSpinners),
  color: 'cyan'
})

const isSupportedLangauge = function (language) {
  if (typeof language === 'boolean') return false
  return language == null ? false : (config.supported_languages.indexOf(language.toLowerCase()) !== -1)
}

const runCommand = function (cmd, callback) {
  let exMessage
  try {
    childProcess.execSync(cmd, {
      stdio: [0, 1, 2]
    })
  } catch (ex) {
    exMessage = ex.message
  } finally {
    callback(exMessage)
  }
}

const runGit = function (cmd, callback) {
  childProcess.exec(cmd, {
    stdio: [0, 1, 2]
  }, function (error, stdout, stderror) {
    callback(error, stdout, stdout)
  })
}

const doespathExist = function (path) {
  try {
    return fs.statSync(path).isDirectory() || fs.statSync(path).isFile()
  } catch (e) {
    if (e.code === 'ENOENT') {
      return false
    } else {
      throw e
    }
  }
}

module.exports = {
  runGit: runGit,
  getSpinner: getSpinner,
  randomJsonValue: randomJsonValue,
  doespathExist: doespathExist,
  isSupportedLangauge: isSupportedLangauge,
  runCommand: runCommand,
  isMacOS: process.platform === 'darwin',
  isWindows: process.platform === 'win32',
  isLinux: process.platform === 'win32'
}
