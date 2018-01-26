# Serviceproto.HaberdasherApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**makeHat**](HaberdasherApi.md#makeHat) | **POST** /twirp/twitch.twirp.example.Haberdasher/MakeHat | MakeHat produces a hat of mysterious, randomly-selected color!


<a name="makeHat"></a>
# **makeHat**
> ExampleHat makeHat(body)

MakeHat produces a hat of mysterious, randomly-selected color!

### Example
```javascript
var Serviceproto = require('serviceproto');

var apiInstance = new Serviceproto.HaberdasherApi();

var body = new Serviceproto.ExampleSize(); // ExampleSize | 


var callback = function(error, data, response) {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
};
apiInstance.makeHat(body, callback);
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**ExampleSize**](ExampleSize.md)|  | 

### Return type

[**ExampleHat**](ExampleHat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

