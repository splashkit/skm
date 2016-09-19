const Table = require('cli-table')
const config = require('../config')
const help = require('generate-help')

const execute = function (argv, callback) {

  let output = help({
    usage: 'skm clang++ [file] -o [output]',
    desc: 'Build a CPP file.',
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
  });

  console.log(output);

  output = help({
    usage: 'skm update',
    desc: 'Updates SKM and SplashKit.',
  });

  console.log(output);
  callback()
}

module.exports = {
  execute: execute
}