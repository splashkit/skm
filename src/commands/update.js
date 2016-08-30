//require simple-git with optional empty working path.
const git = require("nodegit")
const utils = require('../utils')
const logger = require('winston-color')
const config = require('../config')
const cliSpinners = require('cli-spinners')
const ora = require('ora')
const os = require('os')

var repository;

const spinner = utils.getSpinner
spinner.text = 'Updating SplashKit!'

const execute = function(args, callback) {

  const repo = config['splashkit_repo']
  const installPath = `${os.homedir()}/.splashkit/install-test`

  logger.info("Update command was executed. Cloning repo")

  if (!utils.doespathExist(installPath)) {
    callback(Error(`can't find SplashKit, please install splashkit before updating!`))
  } else {

    var repoDir = installPath
    let data

    // console.log(repoDir)
    //
    // //Open a repository that needs to be fetched and fast-forwarded
    // git.Repository.open(repoDir, function(err, repo) {
    //   console.log("Using " + repo)
    //
    //   data = repo.fetchAll({
    //     callbacks: {
    //       credentials: function(url, userName) {
    //         return nodegit.Cred.sshKeyFromAgent(userName);
    //       },
    //       certificateCheck: function() {
    //         return 1;
    //       }
    //     }
    //   });
    //   logger.info(`data is: ${data}`)
    //
    // })
    callback('update not implemented yet')
  }
}

module.exports = {
  execute: execute
}