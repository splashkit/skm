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

const writeDotSplashkit = function (data, path = './') {

  let dataAsString = "// DO NOT TOUCH, GENERATED SPLASHKIT FILE.\n" + JSON.stringify(data, null, "\t")

  console.info("path is: " + path + " data is: " + dataAsString)
  fs.writeFile(path + '.splashkit', dataAsString, (err) => {
    if (err) throw err;
    console.info('Saved to ./.splashkit successfully.');
  });
}


const isSplashkit = function (pathToCheck = './') {
  console.info("Checking for splashkit file at: " + pathToCheck + '.splashkit')
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