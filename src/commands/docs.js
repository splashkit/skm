const {app, BrowserWindow} = require('electron')
const config = require('../config')
const utils = require('../utils')

const execute = function (argv, callback) {
  if (argv == null || argv['_'][1] == null) {
    createWindow()
  } else {
    if (config['doc_to_url'].indexOf(argv['_'][1]) > -1) {
      createWindow(`api/${argv['_'][1]}`)
    } else {
      createWindow()
    }
  }
}
let win

function createWindow (doc = '') {
  // Create the browser window.
  win = new BrowserWindow({backgroundColor: '#3F51B5', width: 1000, height: 800})

  // and load the index.html of the app.
  win.loadURL(`http://splashkit.io/${doc}`)

  // Emitted when the window is closed.
  win.on('closed', () => {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    win = null
  })
}

app.on('activate', () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (win === null) {
    createWindow()
  }
})

module.exports = {
  execute: execute
}
