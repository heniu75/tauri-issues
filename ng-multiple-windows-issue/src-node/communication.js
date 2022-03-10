// this is a node console application
const readline = require('readline')

module.exports = {

  onMessageFromRustServer(cb) {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
      terminal: false
    })

    rl.on('line', function (line) {
      cb(line)
    })
  },

  write(message) {
    console.log(message)
  }
}
