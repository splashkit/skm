const os = require('os')
const compilers = require(`${os.homedir()}/.splashkit/splashkit-macos/compilers`)
const utils = require(`../../utils`)
const mkdirp = require('mkdirp')

const _getCompiler = function(compilerName) {
  return compilers.get(compilerName)
}

const hasCompilerNamed = function(compilerName) {
  return _getCompiler(compilerName) != null
}

const execute = function(argv, callback) {
  const compilerName = argv['_'][0]
  const compiler = _getCompiler(compilerName)

  compiler.execute(argv, function (err) {
    if (err) {
      callback(err)
    } else {
      callback(null, 'Successfully compiled! 🎉')
    }
  })
}

module.exports = {
  execute: execute,
  hasCompilerNamed: hasCompilerNamed
}