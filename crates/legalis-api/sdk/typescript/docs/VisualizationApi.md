# VisualizationApi

All URIs are relative to *http://localhost:3000*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**visualizeStatute**](#visualizestatute) | **GET** /api/v1/visualize/{id} | Visualize a statute|

# **visualizeStatute**
> VisualizationResponse visualizeStatute()

Generate a visual representation of a statute in various formats (SVG, Mermaid, PlantUML, DOT, ASCII, HTML)

### Example

```typescript
import {
    VisualizationApi,
    Configuration
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new VisualizationApi(configuration);

let id: string; //Statute ID to visualize (default to undefined)
let format: 'svg' | 'mermaid' | 'plantuml' | 'dot' | 'ascii' | 'html'; //Output format (optional) (default to 'svg')
let theme: 'light' | 'dark' | 'high_contrast' | 'colorblind_friendly'; //Color theme (optional) (default to 'light')

const { status, data } = await apiInstance.visualizeStatute(
    id,
    format,
    theme
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | Statute ID to visualize | defaults to undefined|
| **format** | [**&#39;svg&#39; | &#39;mermaid&#39; | &#39;plantuml&#39; | &#39;dot&#39; | &#39;ascii&#39; | &#39;html&#39;**]**Array<&#39;svg&#39; &#124; &#39;mermaid&#39; &#124; &#39;plantuml&#39; &#124; &#39;dot&#39; &#124; &#39;ascii&#39; &#124; &#39;html&#39;>** | Output format | (optional) defaults to 'svg'|
| **theme** | [**&#39;light&#39; | &#39;dark&#39; | &#39;high_contrast&#39; | &#39;colorblind_friendly&#39;**]**Array<&#39;light&#39; &#124; &#39;dark&#39; &#124; &#39;high_contrast&#39; &#124; &#39;colorblind_friendly&#39;>** | Color theme | (optional) defaults to 'light'|


### Return type

**VisualizationResponse**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Visualization generated successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

