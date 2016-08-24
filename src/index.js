const app = require('electron').app;
const cli = require('./cli');
const winston = require('winston-color');

const argPos = 2;

const isMacOS = (process.platform === 'darwin');

// On macOS we want to make sure we never show the app icon unless
// GUI is specifically requested.
if (isMacOS) {
  app.dock.hide();
}

app.on('ready', function () {

  winston.info('SKM is ready...');

  // TODO: check if development or runtime.
  const cmdName = process.argv[argPos]

  //args will be left with everything after the command itself.
  const args = process.argv.slice(argPos + 1);

  var useCLI = cmdName != null
  winston.info("are we using the CLI?: " + useCLI)

  if (useCLI) {
    let callback = function (err, data) { };
    cli.execute(cmdName, args, callback);
  }
  else {
    {
      //TODO: Open Window
    }
  }
  app.quit();
});
