# legalis_client.StatutesApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_statute**](StatutesApi.md#create_statute) | **POST** /api/v1/statutes | Create a new statute
[**delete_statute**](StatutesApi.md#delete_statute) | **DELETE** /api/v1/statutes/{id} | Delete a statute
[**get_statute**](StatutesApi.md#get_statute) | **GET** /api/v1/statutes/{id} | Get a statute by ID
[**list_statutes**](StatutesApi.md#list_statutes) | **GET** /api/v1/statutes | List all statutes


# **create_statute**
> CreateStatute201Response create_statute(create_statute_request)

Create a new statute

Creates a new statute in the system

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.create_statute201_response import CreateStatute201Response
from legalis_client.models.create_statute_request import CreateStatuteRequest
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
    api_instance = legalis_client.StatutesApi(api_client)
    create_statute_request = {"statute":{"effect":{"description":"Person gains capacity to enter into contracts","effect_type":"Grant","parameters":{"right":"contract_capacity"}},"id":"civil-code-article-42","preconditions":[{"condition_type":"Age","operator":"GreaterThanOrEqual","value":18}],"title":"Contractual Capacity","version":1}} # CreateStatuteRequest | 

    try:
        # Create a new statute
        api_response = api_instance.create_statute(create_statute_request)
        print("The response of StatutesApi->create_statute:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling StatutesApi->create_statute: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_statute_request** | [**CreateStatuteRequest**](CreateStatuteRequest.md)|  | 

### Return type

[**CreateStatute201Response**](CreateStatute201Response.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Statute created successfully |  -  |
**400** | Invalid request |  -  |
**401** | Missing or invalid authentication credentials |  -  |
**403** | Insufficient permissions to create statutes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_statute**
> delete_statute(id)

Delete a statute

Deletes a statute from the system

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
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
    api_instance = legalis_client.StatutesApi(api_client)
    id = 'civil-code-art-1' # str | Statute ID to delete

    try:
        # Delete a statute
        api_instance.delete_statute(id)
    except Exception as e:
        print("Exception when calling StatutesApi->delete_statute: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| Statute ID to delete | 

### Return type

void (empty response body)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**204** | Statute deleted successfully |  -  |
**401** | Missing or invalid authentication credentials |  -  |
**403** | Insufficient permissions to delete statutes |  -  |
**404** | Statute not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_statute**
> CreateStatute201Response get_statute(id)

Get a statute by ID

Returns detailed information about a specific statute

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.create_statute201_response import CreateStatute201Response
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
    api_instance = legalis_client.StatutesApi(api_client)
    id = 'civil-code-art-1' # str | Statute ID

    try:
        # Get a statute by ID
        api_response = api_instance.get_statute(id)
        print("The response of StatutesApi->get_statute:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling StatutesApi->get_statute: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| Statute ID | 

### Return type

[**CreateStatute201Response**](CreateStatute201Response.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Statute details |  -  |
**401** | Missing or invalid authentication credentials |  -  |
**403** | Insufficient permissions to read statutes |  -  |
**404** | Statute not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_statutes**
> StatuteListResponse list_statutes()

List all statutes

Returns a list of all statutes with summary information

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.statute_list_response import StatuteListResponse
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
    api_instance = legalis_client.StatutesApi(api_client)

    try:
        # List all statutes
        api_response = api_instance.list_statutes()
        print("The response of StatutesApi->list_statutes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling StatutesApi->list_statutes: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**StatuteListResponse**](StatuteListResponse.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List of statutes |  -  |
**401** | Missing or invalid authentication credentials |  -  |
**403** | Insufficient permissions to read statutes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

