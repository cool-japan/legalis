# legalis_client.VerificationApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**verify_statutes**](VerificationApi.md#verify_statutes) | **POST** /api/v1/verify | Verify statutes


# **verify_statutes**
> VerifyStatutes200Response verify_statutes(verify_request)

Verify statutes

Verifies one or more statutes for logical consistency and validity

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.verify_request import VerifyRequest
from legalis_client.models.verify_statutes200_response import VerifyStatutes200Response
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
    api_instance = legalis_client.VerificationApi(api_client)
    verify_request = {"statute_ids":[]} # VerifyRequest | 

    try:
        # Verify statutes
        api_response = api_instance.verify_statutes(verify_request)
        print("The response of VerificationApi->verify_statutes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VerificationApi->verify_statutes: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **verify_request** | [**VerifyRequest**](VerifyRequest.md)|  | 

### Return type

[**VerifyStatutes200Response**](VerifyStatutes200Response.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Verification results |  -  |
**400** | Invalid request |  -  |
**401** | Missing or invalid authentication credentials |  -  |
**403** | Insufficient permissions to verify statutes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

