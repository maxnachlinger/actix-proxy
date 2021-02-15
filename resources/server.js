const http = require('http');

http.createServer((req, res) => {
  console.log(req.headers);

  res.writeHead(200, {'content-type': 'application/json'});
  res.end(JSON.stringify({ message: 'test' }));
}).listen(8000);
