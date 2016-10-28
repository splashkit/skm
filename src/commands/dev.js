const utils = require('../utils')
const inquirer = require('inquirer')
const commandExists = require('command-exists')

let questions = [
  {
    type: 'input',
    name: 'splashkit_path',
    message: 'Enter the full path to where you would like to install the SplashKit development tools.'
  }
]

const installBrew = function (callback) {
  commandExists('brew', function (err, commandExists) {
    if (err) {
      callback(err)
    } else if (!commandExists) {
      // would you like to install brew?
      utils.runCommand('/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"', function (err) {
        callback(err)
      })
    } else {
      callback()
    }
  })
}

const preExecuteOnCLI = function (argv, callback) {
  let path = argv['p'] || argv['path']

  // just ask for name
  if (path != null) {
    callback(null, argv)
  } else {
    inquirer.prompt(questions).then(function (answers) {
      if (answers.splashkit_path != null) {
        path = answers['splashkit_path']
      }

      argv['p'] = argv['path'] = path

      callback(null, argv)
    })
  }
}

const execute = function (argv, callback) {
  const path = argv['p'] || argv['path']

  // check is valid path
  if (path == null) {
    return callback(Error('No path specified, Use --path or -p to specify one.'))
  }

  installBrew(function (err) {
    if (err) {
      return callback(err)
    } else {
      utils.runGit(`git clone -b develop --recursive https://github.com/jakerenzella/splashkit.git ${path}`, function (err) {
        if (err) {
          callback(err)
        } else {
          callback(null, 'successfully installed splashkit')
        }
      })
    }
  })
}

module.exports = {
  execute: execute,
  preExecuteOnCLI: preExecuteOnCLI
}
