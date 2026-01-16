# legalis_client.MetricsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metrics**](MetricsApi.md#metrics) | **GET** /metrics | Prometheus metrics


# **metrics**
> str metrics()

Prometheus metrics

Returns metrics in Prometheus format for monitoring

### Example


```python
import legalis_client
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
    api_instance = legalis_client.MetricsApi(api_client)

    try:
        # Prometheus metrics
        api_response = api_instance.metrics()
        print("The response of MetricsApi->metrics:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling MetricsApi->metrics: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Prometheus metrics |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

