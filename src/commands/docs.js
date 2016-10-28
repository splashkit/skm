const {app, BrowserWindow} = require('electron')
const config = require('../config')

const execute = function (argv, callback) {
  const doc = argv['_'][1]
  if (config['doc_to_url'].indexOf(doc) > -1) {
    createWindow(`api/${doc}`)
  } else {
    createWindow()
  }
}
let win

function createWindow (doc = '') {
  // Create the browser window.
  win = new BrowserWindow({backgroundColor: '##3F51B5', width: 1000, height: 800, frame: false})
  // and load the index.html of the app.
  win.loadURL(`http://localhost:4567/${doc}`)

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
