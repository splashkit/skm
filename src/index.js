const app = require('electron').app;
const cli = require('./cli');

const isMacOS = (process.platform === 'darwin');

// On macOS we want to make sure we never show the app icon unless
// GUI is specifically requested.
if (isMacOS) {
  app.dock.hide();
}

app.on('ready', function () {
  console.log('SKM is ready...');
  console.log('argument 1 was :' + process.argv[2]);
  
  // TODO: Check if arguments are provided, if so then use CLI
  // otherwise request open GUI
  var useCLI = true;
  if (useCLI) {
    let args = [];
    let callback = function (err, data) {
    };

    cli.execute(process.argv[2], callback);
  }
  app.quit();
});
