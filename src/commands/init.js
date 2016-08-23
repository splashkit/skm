var fs = require('fs');

const preExecuteOnCLI = function () {
  //read from CLI
  return [];
}

const execute = function(args, callback) {
  if (isMacOS) {
    fs.writeFile("./", ".splashkit", function(err) {
    if(err) {
        return console.log(err);
    }
    console.log("The file was saved!");
});

}

    return
  }

module.exports = {
  execute: execute
}
