const http = require('http');

http.createServer((req, res) => {
  res.writeHead(200, {'content-type': 'application/json'});
  res.end(JSON.stringify(req.headers));
}).listen(8000);
