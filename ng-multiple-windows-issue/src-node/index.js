const { write, onMessageFromRustServer } = require('./communication')

const fs = require('fs');
const os = require("os");

onMessageFromRustServer((line) => {
  write(`(NodeApp) The node app has received a message from the rust server...`)
  write(`(NodeApp) ${line}`)
  fs.appendFile(`app-messages-received-from-rust-server.txt`, `${line}${os.EOL}`, function (err) {
    //     if (err) throw err;
    console.warn('Ignoring error...', err);
  });
})

 setInterval(() => {
   write(`(NodeApp): [${new Date().toLocaleTimeString()}] something happened in the node application!`)
 }, 3000)
