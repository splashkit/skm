const app = require('electron').app;
const cli = require('./cli');
const logger = require('winston-color');

let argPos;

console.log(process.env.NODE_ENV)
process.env.NODE_ENV == 'development' ? argPos = 2 : argPos = 1

const isMacOS = (process.platform === 'darwin');

// On macOS we want to make sure we never show the app icon unless
// GUI is specifically requested.
if (isMacOS) {
  app.dock.hide();
}

app.on('ready', function () {

  logger.info('SplashKit Manager\n');

  // check if development or runtime.
  const cmdName = process.argv[argPos]

  //args will be left with everything after the command itself.
  const args = process.argv.slice(argPos + 1);

  var useCLI = cmdName != null

  if (useCLI) {
    logger.info(`using SKM CLI with ${argPos}`)
    let callback = function (err, data) { };
    cli.execute(cmdName, args, callback);
  }
  else {
    //TODO: Open Window
  }
  
  app.quit();
});
