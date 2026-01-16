# legalis_client.HealthApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health_check**](HealthApi.md#health_check) | **GET** /health | Health check


# **health_check**
> HealthCheck200Response health_check()

Health check

Returns the service health status

### Example


```python
import legalis_client
from legalis_client.models.health_check200_response import HealthCheck200Response
from legalis_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:3000
# See configuration.py for a list of all supported configuration parameters.
configuration = legalis_client.Configuration(
    host = "http://localhost:3000"
)


# Enter a context with an instance of the API client
with legalis_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = legalis_client.HealthApi(api_client)

    try:
        # Health check
        api_response = api_instance.health_check()
        print("The response of HealthApi->health_check:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling HealthApi->health_check: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**HealthCheck200Response**](HealthCheck200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Service is healthy |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

