var fs = require('fs');
const logger = require('winston-color');
const jsonminify = require("jsonminify");
var execSync = require('child_process').execSync;

const generateDotSplashkit = function () {
  const data = {
    "version": '-1',
    "dateCreated": new Date(),
    "message": 'Jake',
    "status": 'initialised'
  }
  return data
}

const runCommand = function (cmd){
  execSync(cmd, {stdio:[0,1,2]})
}

const readDotSplashkit = function (path = './') {
  const data = fs.readFileSync(path + '.splashkit', 'utf8')
  return JSON.parse(JSON.minify(data))
}

const writeDotSplashkit = function (data, path = './') {
  let dataAsString = "//DO NOT TOUCH, THIS IS A GENERATED SPLASHKIT FILE.\n\n" + JSON.stringify(data, null, "\t")

  logger.info("path is: " + path + " data is: " + dataAsString)
  fs.writeFileSync(path + '.splashkit', dataAsString)
  logger.info('Saved to ./.splashkit successfully.')
}

const isSplashkit = function (pathToCheck = './') {
  logger.info("Checking for splashkit file at: " + pathToCheck + '.splashkit')
  try {
    return fs.statSync(pathToCheck + '.splashkit').isFile();
  } catch (e) {
    if (e.code === 'ENOENT') {
      return false;
    } else {
      throw e;
    }
  }
}

module.exports = {
  runCommand: runCommand,
  readDotSplashkit: readDotSplashkit,
  writeDotSplashkit: writeDotSplashkit,
  generateDotSplashkit: generateDotSplashkit,
  isSplashkit: isSplashkit,
  isMacOS: process.platform === 'darwin',
  isWindows: process.platform === 'win32',
  isLinux: process.platform === 'win32'
}