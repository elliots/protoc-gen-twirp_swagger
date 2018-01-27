'use strict';

var utils = require('../utils/writer.js');
var Haberdasher = require('../service/HaberdasherService');

module.exports.makeHat = function makeHat (req, res, next) {
  var body = req.swagger.params['body'].value;
  Haberdasher.makeHat(body)
    .then(function (response) {
      utils.writeJson(res, response);
    })
    .catch(function (response) {
      utils.writeJson(res, response);
    });
};
