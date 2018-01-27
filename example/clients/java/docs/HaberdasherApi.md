# HaberdasherApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**makeHat**](HaberdasherApi.md#makeHat) | **POST** /twirp/twitch.twirp.example.Haberdasher/MakeHat | MakeHat produces a hat of mysterious, randomly-selected color!


<a name="makeHat"></a>
# **makeHat**
> ExampleHat makeHat(body)

MakeHat produces a hat of mysterious, randomly-selected color!

### Example
```java
// Import classes:
//import io.swagger.client.ApiException;
//import io.swagger.client.api.HaberdasherApi;


HaberdasherApi apiInstance = new HaberdasherApi();
ExampleSize body = new ExampleSize(); // ExampleSize | 
try {
    ExampleHat result = apiInstance.makeHat(body);
    System.out.println(result);
} catch (ApiException e) {
    System.err.println("Exception when calling HaberdasherApi#makeHat");
    e.printStackTrace();
}
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

