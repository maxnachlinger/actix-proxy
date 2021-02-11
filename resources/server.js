const http = require('http');

http.createServer((req, res) => {
  res.writeHead(200);
  console.log(req.headers);
  res.end('OK');
}).listen(8000);
