const help = require('generate-help')

const execute = function (argv, callback) {
  let output = help({
    usage: 'skm <command>',
    desc: 'Runs the SplashKit Manager.',
    options: {
      name: {
        alias: 'n',
        desc: 'The name of the SplashKit project'
      },
      language: {
        aliases: ['l'],
        desc: 'The language of the SplashKit project'
      }
    }
  })

  console.log(output)
  callback()
}

module.exports = {
  execute: execute
}
