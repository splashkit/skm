const isMacOS = (process.platform === 'darwin');

const execute = function(args, callback) {
  if (isMacOS) {
    console.log("Mac Install command was executed")
    //git clone
    return
  }
}

module.exports = {
  execute: execute
}
