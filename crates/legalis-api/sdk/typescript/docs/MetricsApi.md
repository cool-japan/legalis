# MetricsApi

All URIs are relative to *http://localhost:3000*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**metrics**](#metrics) | **GET** /metrics | Prometheus metrics|

# **metrics**
> string metrics()

Returns metrics in Prometheus format for monitoring

### Example

```typescript
import {
    MetricsApi,
    Configuration
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new MetricsApi(configuration);

const { status, data } = await apiInstance.metrics();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Prometheus metrics |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

