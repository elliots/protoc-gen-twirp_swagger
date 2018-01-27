# SwaggerClient::HaberdasherApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**make_hat**](HaberdasherApi.md#make_hat) | **POST** /twirp/twitch.twirp.example.Haberdasher/MakeHat | MakeHat produces a hat of mysterious, randomly-selected color!


# **make_hat**
> ExampleHat make_hat(body)

MakeHat produces a hat of mysterious, randomly-selected color!

### Example
```ruby
# load the gem
require 'swagger_client'

api_instance = SwaggerClient::HaberdasherApi.new

body = SwaggerClient::ExampleSize.new # ExampleSize | 


begin
  #MakeHat produces a hat of mysterious, randomly-selected color!
  result = api_instance.make_hat(body)
  p result
rescue SwaggerClient::ApiError => e
  puts "Exception when calling HaberdasherApi->make_hat: #{e}"
end
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



