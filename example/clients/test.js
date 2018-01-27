var Serviceproto = require('./javascript');

var devClient = new Serviceproto.ApiClient();
devClient.basePath = 'http://localhost:8080';

var api = new Serviceproto.HaberdasherApi(devClient);

var body = new Serviceproto.ExampleSize(); // {ExampleSize} 
body.inches = 20;

var callback = function(error, data, response) {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ', data);
  }
};
api.makeHat(body, callback);