const app = require('electron').app;
const cli = require('./cli');
const gui = require('./gui');
const minimist = require('minimist')
const utils = require('./utils')

let argv, isGuiStart;

const prepare = function () {
  let argPos = process.env.NODE_ENV == 'development' ? 2 : 1
  argv = minimist(process.argv.slice(argPos))
  isGuiStart = argv['_'].length == 0
  if (utils.isMacOS) {
    app.dock.hide();
  }
}

const guiStart = function () {
  gui.execute()
}

const cliStart = function () {
  let cmdName = argv['_'][0]
  console.info(argv)
  cli.execute(cmdName, argv)
  app.quit()
}

prepare();
app.on('ready', isGuiStart ? guiStart : cliStart)
