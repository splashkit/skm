//require simple-git with optional empty working path.
const Git = require("nodegit");
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')

const preExecuteOnCLI = function () {
  //read from CLI
  return []
}

const execute = function(args, callback) {
  if (utils.isMacOS) {
    logger.log("Before CLONE")


    const repo = config['splashkit_repo']
    const installPath = config['splashkit_install_location']

    logger.info("Mac Install command was executed. Cloning ")

    if (utils.doespathExist(installPath)) {
      return callback(Error(`can't install, splashkit is already installed!`))
    }

    // Clone a given repository into the `./tmp` folder.
    Git.Clone(repo, installPath)
      // Look up this known commit.
      .then(function(repo) {
        // Use a known commit sha from this repository.
        logger.log("IN THEN OF CLONE")
        return repo.getCommit("59b20b8d5c6ff8d09518454d4dd8b7b30f095ab5");
      })
      .catch(function(err) { console.log(err); });
  }
}

module.exports = {
  execute: execute,
  preExecuteOnCLI: preExecuteOnCLI
}
