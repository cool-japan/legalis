# legalis_client.VisualizationApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**visualize_statute**](VisualizationApi.md#visualize_statute) | **GET** /api/v1/visualize/{id} | Visualize a statute


# **visualize_statute**
> VisualizationResponse visualize_statute(id, format=format, theme=theme)

Visualize a statute

Generate a visual representation of a statute in various formats (SVG, Mermaid, PlantUML, DOT, ASCII, HTML)

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.visualization_response import VisualizationResponse
from legalis_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:3000
# See configuration.py for a list of all supported configuration parameters.
configuration = legalis_client.Configuration(
    host = "http://localhost:3000"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: ApiKeyHeader
configuration.api_key['ApiKeyHeader'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyHeader'] = 'Bearer'

# Configure API key authorization: ApiKeyAuth
configuration.api_key['ApiKeyAuth'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyAuth'] = 'Bearer'

# Configure Bearer authorization (JWT): BearerAuth
configuration = legalis_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with legalis_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = legalis_client.VisualizationApi(api_client)
    id = 'civil-code-art-1' # str | Statute ID to visualize
    format = svg # str | Output format (optional) (default to svg)
    theme = light # str | Color theme (optional) (default to light)

    try:
        # Visualize a statute
        api_response = api_instance.visualize_statute(id, format=format, theme=theme)
        print("The response of VisualizationApi->visualize_statute:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VisualizationApi->visualize_statute: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| Statute ID to visualize | 
 **format** | **str**| Output format | [optional] [default to svg]
 **theme** | **str**| Color theme | [optional] [default to light]

### Return type

[**VisualizationResponse**](VisualizationResponse.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Visualization generated successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

