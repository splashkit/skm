const app = require('electron').app;

// On macOS we want to make sure we never show the app icon unless
// GUI is specifically requested.
if (process.platform === 'darwin') {
  app.dock.hide();
}

app.on('ready', function () {
  // TODO: Check if arguments are provided, if so then use CLI
  // otherwise request open GUI
  console.log('App ready');
  app.quit();
});
