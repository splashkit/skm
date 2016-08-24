var fs = require('fs');
const winston = require('winston-color');

const generateDotSplashkit = function () {
  const data = {
    "version": '-1',
    "dateCreated": new Date(),
    "message": 'Jake',
    "status": 'initialised'
  }
  return data
}

const writeDotSplashkit = function (data, path = './') {

  let dataAsString = "//DO NOT TOUCH, THIS IS A GENERATED SPLASHKIT FILE.\n\n" + JSON.stringify(data, null, "\t")

  winston.info("path is: " + path + " data is: " + dataAsString)
  fs.writeFile(path + '.splashkit', dataAsString, (err) => {
    if (err) throw err;
    winston.info('Saved to ./.splashkit successfully.');
  });
}


const isSplashkit = function (pathToCheck = './') {
  winston.info("Checking for splashkit file at: " + pathToCheck + '.splashkit')
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
  writeDotSplashkit: writeDotSplashkit,
  generateDotSplashkit: generateDotSplashkit,
  isSplashkit: isSplashkit,
  isMacOS: process.platform === 'darwin',
  isWindows: process.platform === 'win32',
  isLinux: process.platform === 'win32'
}