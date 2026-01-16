# StatutesApi

All URIs are relative to *http://localhost:3000*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createStatute**](#createstatute) | **POST** /api/v1/statutes | Create a new statute|
|[**deleteStatute**](#deletestatute) | **DELETE** /api/v1/statutes/{id} | Delete a statute|
|[**getStatute**](#getstatute) | **GET** /api/v1/statutes/{id} | Get a statute by ID|
|[**listStatutes**](#liststatutes) | **GET** /api/v1/statutes | List all statutes|

# **createStatute**
> CreateStatute201Response createStatute(createStatuteRequest)

Creates a new statute in the system

### Example

```typescript
import {
    StatutesApi,
    Configuration,
    CreateStatuteRequest
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new StatutesApi(configuration);

let createStatuteRequest: CreateStatuteRequest; //

const { status, data } = await apiInstance.createStatute(
    createStatuteRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createStatuteRequest** | **CreateStatuteRequest**|  | |


### Return type

**CreateStatute201Response**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Statute created successfully |  -  |
|**400** | Invalid request |  -  |
|**401** | Missing or invalid authentication credentials |  -  |
|**403** | Insufficient permissions to create statutes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteStatute**
> deleteStatute()

Deletes a statute from the system

### Example

```typescript
import {
    StatutesApi,
    Configuration
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new StatutesApi(configuration);

let id: string; //Statute ID to delete (default to undefined)

const { status, data } = await apiInstance.deleteStatute(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | Statute ID to delete | defaults to undefined|


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
|**204** | Statute deleted successfully |  -  |
|**401** | Missing or invalid authentication credentials |  -  |
|**403** | Insufficient permissions to delete statutes |  -  |
|**404** | Statute not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getStatute**
> CreateStatute201Response getStatute()

Returns detailed information about a specific statute

### Example

```typescript
import {
    StatutesApi,
    Configuration
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new StatutesApi(configuration);

let id: string; //Statute ID (default to undefined)

const { status, data } = await apiInstance.getStatute(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | Statute ID | defaults to undefined|


### Return type

**CreateStatute201Response**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Statute details |  -  |
|**401** | Missing or invalid authentication credentials |  -  |
|**403** | Insufficient permissions to read statutes |  -  |
|**404** | Statute not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listStatutes**
> StatuteListResponse listStatutes()

Returns a list of all statutes with summary information

### Example

```typescript
import {
    StatutesApi,
    Configuration
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new StatutesApi(configuration);

const { status, data } = await apiInstance.listStatutes();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**StatuteListResponse**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List of statutes |  -  |
|**401** | Missing or invalid authentication credentials |  -  |
|**403** | Insufficient permissions to read statutes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

