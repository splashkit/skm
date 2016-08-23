const createSKJSON = function () {

  var data = {
    "version":'-1',
    "dateCreated":new Date(),
    "message": 'Jake'
  }

  //format the json string so it doesn't stay on one line
  return JSON.stringify(data, null, "\t");
}

module.exports = {
  createSKJSON: createSKJSON,
  isMacOS: process.platform === 'darwin',
  isWindows: process.platform === 'win32',
  isLinux: process.platform === 'win32'
}