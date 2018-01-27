'use strict';


/**
 * MakeHat produces a hat of mysterious, randomly-selected color!
 *
 * body ExampleSize 
 * returns exampleHat
 **/
exports.makeHat = function(body) {
  return new Promise(function(resolve, reject) {
    var examples = {};
    examples['application/json'] = {
  "size" : 0,
  "color" : "color",
  "name" : "name"
};
    if (Object.keys(examples).length > 0) {
      resolve(examples[Object.keys(examples)[0]]);
    } else {
      resolve();
    }
  });
}

