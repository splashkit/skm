//require simple-git with optional empty working path.
var git = require('simple-git')(  );

const isMacOS = (process.platform === 'darwin');

const execute = function(args, callback) {
  if (isMacOS) {
    console.log("Mac Install command was executed. Cloning ");
    git.clone("https://github.com/splashkit/splashkit", "./splashkit-download")
    return
  }
}

module.exports = {
  execute: execute
}
