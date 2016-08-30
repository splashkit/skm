var fs = require('fs')
const logger = require('winston-color')
const jsonminify = require("jsonminify")
const config = require("../config")
const {execSync} = require('child_process')
const ora = require('ora')
const cliSpinners = require('cli-spinners')

const randomJsonValue = function (obj) {
    const keys = Object.keys(obj)
    return obj[keys[keys.length * Math.random() << 0]]
}

const getSpinner = new ora({
  spinner: randomJsonValue(cliSpinners),
  color: 'cyan'
})

const isSupportedLangauge = function (language) {
  if (typeof language === "boolean") return false
  return language == null ? false : (config.supported_languages.indexOf(language.toLowerCase()) != -1)
}

const runCommand = function (cmd, callback){
  try {
    execSync(cmd, {stdio:[0,1,2]})
  }
  catch (ex) {
    callback(ex.message)
  }
}

const doespathExist = function (path) {
  logger.debug(`Checking for directory or file at: ${path}`)
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
  getSpinner: getSpinner,
  randomJsonValue: randomJsonValue,
  doespathExist: doespathExist,
  isSupportedLangauge: isSupportedLangauge,
  runCommand: runCommand,
  isMacOS: process.platform === 'darwin',
  isWindows: process.platform === 'win32',
  isLinux: process.platform === 'win32'
}
