# IO.Swagger.Api.HaberdasherApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**MakeHat**](HaberdasherApi.md#makehat) | **POST** /twirp/twitch.twirp.example.Haberdasher/MakeHat | MakeHat produces a hat of mysterious, randomly-selected color!


<a name="makehat"></a>
# **MakeHat**
> ExampleHat MakeHat (ExampleSize body)

MakeHat produces a hat of mysterious, randomly-selected color!

### Example
```csharp
using System;
using System.Diagnostics;
using IO.Swagger.Api;
using IO.Swagger.Client;
using IO.Swagger.Model;

namespace Example
{
    public class MakeHatExample
    {
        public void main()
        {
            var apiInstance = new HaberdasherApi();
            var body = new ExampleSize(); // ExampleSize | 

            try
            {
                // MakeHat produces a hat of mysterious, randomly-selected color!
                ExampleHat result = apiInstance.MakeHat(body);
                Debug.WriteLine(result);
            }
            catch (Exception e)
            {
                Debug.Print("Exception when calling HaberdasherApi.MakeHat: " + e.Message );
            }
        }
    }
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

