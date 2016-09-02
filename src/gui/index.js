const Table = require('cli-table')
const config = require('../config')
const help = require('generate-help')

var table = new Table();

const execute = function (argv, callback) {
  // table is an Array, so you can `push`, `unshift`, `splice` and friends
  // table.push(
  //     ['update', '', 'Update SplashKit and SplashKit Manager' ],
  //     ['init', '-l, -n', 'Initialise a SplashKit project language -l, name -n'],
  //     ['clang++', '...', 'Build a c/cpp program with SplashKit and standard clang arguments'],
  //     ['dev', '-p', 'Download tools to develop SplashKit ']
  // );
  //
  // console.log(`\nSplashKit Manager: https://www.github.com/splashkit/skm`);
  // console.log(`Version: ${config['splashkit_version']}\n\n\t\t\t\t\tAvailable Commands`)
  // console.log(table.toString());
  // callback()


  // TODO: Finish this
  let output = help({
    usage: 'skm init [options]',
    desc: 'Initialises a SplashKit project',
    options: {
      name: {
        alias: 'n',
        desc: 'The name of the SplashKit project'
      },

      language: {
        aliases: ['l'],
        desc: 'The language of the SplashKit project'
      }
    },
    commands: [{
      name: 'hi',
      desc: 'Say hi'
    }, {
      name: 'yo',
      desc: 'Say yo'
    }]
  });

  console.log(output);

}

module.exports = {
  execute: execute
}