const os = require('os')
const utils = require('../../utils')
const logger = require('winston-color')

const _getCompiler = function (compilerName) {
  let compilers
  const home = process.env.HOME
  try {
    compilers = require(`${home}/.splashkit/compilers`)
  } catch (error) {
    logger.error(error.message)
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

  // check for sigabort
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
