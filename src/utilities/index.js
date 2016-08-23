var fs = require('fs');

const generateDotSplashkit = function () {
  const data = {
    "version": '-1',
    "dateCreated": new Date(),
    "message": 'Jake',
    "status": 'initialised'
  }
  return data
}

const isSplashkit = function (pathToCheck = './') {
  console.log("Checking for splashkit file at: " + pathToCheck + '.splashkit')

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
  generateDotSplashkit: generateDotSplashkit,
  isSplashkit: isSplashkit,
  isMacOS: process.platform === 'darwin',
  isWindows: process.platform === 'win32',
  isLinux: process.platform === 'win32'
}