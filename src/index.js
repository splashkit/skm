const app = require('electron').app;
const cli = require('./cli');

// const gui = require('./gui');

let argPos;

console.log(process.env.NODE_ENV)
process.env.NODE_ENV == 'development' ? argPos = 2 : argPos = 1

const argv = require('minimist')(process.argv.slice(argPos))
console.log((process.argv.slice(argPos)))
console.log(argv)

const isMacOS = (process.platform === 'darwin');

// On macOS we want to make sure we never show the app icon unless
// GUI is specifically requested.
if (isMacOS) {
  app.dock.hide();
}

app.on('ready', function () {

  // check if development or runtime.
  const cmdName = process.argv[argPos]

  //args will be left with everything after the command itself.
  const args = process.argv.slice(argPos + 1);

  let useCLI = cmdName != null

  if (useCLI) {
    let callback = function (err, data) {

    };
    cli.execute(cmdName, args, callback);
  }
  else {
    //TODO: Open Window
  }

  app.quit();
});
