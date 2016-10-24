const os = require('os')
const utils = require('../../utils')

const _getCompiler = function (compilerName) {
  let compilers
  try {
    if (utils.isMacOS) {
      compilers = require(`${os.homedir()}/.splashkit/splashkit-macos/compilers`)
    } else if (utils.isWindows) {
      compilers = require(`${os.homedir()}/.splashkit/splashkit-windows/compilers`)
    }
  } catch (error) {
    return // Return null if you can't find the directory.
  }
  return compilers.get(compilerName)
}

const hasCompilerNamed = function (compilerName) {
  return _getCompiler(compilerName) != null
}

const execute = function (argv, callback) {
  const compilerName = argv['_'][0]
  const compiler = _getCompiler(compilerName)

  try {
    compiler.execute(argv, function (err) {
      if (err) {
        callback(err)
      } else {
        callback(null, 'Successfully compiled! 🎉')
      }
    })
  } catch (error) {
    callback(error)
  }
}

module.exports = {
  execute: execute,
  hasCompilerNamed: hasCompilerNamed
}
