# VerificationApi

All URIs are relative to *http://localhost:3000*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**verifyStatutes**](#verifystatutes) | **POST** /api/v1/verify | Verify statutes|

# **verifyStatutes**
> VerifyStatutes200Response verifyStatutes(verifyRequest)

Verifies one or more statutes for logical consistency and validity

### Example

```typescript
import {
    VerificationApi,
    Configuration,
    VerifyRequest
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new VerificationApi(configuration);

let verifyRequest: VerifyRequest; //

const { status, data } = await apiInstance.verifyStatutes(
    verifyRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **verifyRequest** | **VerifyRequest**|  | |


### Return type

**VerifyStatutes200Response**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Verification results |  -  |
|**400** | Invalid request |  -  |
|**401** | Missing or invalid authentication credentials |  -  |
|**403** | Insufficient permissions to verify statutes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

