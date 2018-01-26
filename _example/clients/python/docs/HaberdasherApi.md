# swagger_client.HaberdasherApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**make_hat**](HaberdasherApi.md#make_hat) | **POST** /twirp/twitch.twirp.example.Haberdasher/MakeHat | MakeHat produces a hat of mysterious, randomly-selected color!


# **make_hat**
> ExampleHat make_hat(body)

MakeHat produces a hat of mysterious, randomly-selected color!

### Example
```python
from __future__ import print_function
import time
import swagger_client
from swagger_client.rest import ApiException
from pprint import pprint

# create an instance of the API class
api_instance = swagger_client.HaberdasherApi()
body = swagger_client.ExampleSize() # ExampleSize | 

try:
    # MakeHat produces a hat of mysterious, randomly-selected color!
    api_response = api_instance.make_hat(body)
    pprint(api_response)
except ApiException as e:
    print("Exception when calling HaberdasherApi->make_hat: %s\n" % e)
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

