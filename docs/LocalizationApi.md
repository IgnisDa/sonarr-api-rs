# \LocalizationApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_localization_get**](LocalizationApi.md#api_v3_localization_get) | **GET** /api/v3/localization | 
[**api_v3_localization_id_get**](LocalizationApi.md#api_v3_localization_id_get) | **GET** /api/v3/localization/{id} | 
[**api_v3_localization_language_get**](LocalizationApi.md#api_v3_localization_language_get) | **GET** /api/v3/localization/language | 



## api_v3_localization_get

> models::LocalizationResource api_v3_localization_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LocalizationResource**](LocalizationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_localization_id_get

> models::LocalizationResource api_v3_localization_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::LocalizationResource**](LocalizationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_localization_language_get

> models::LocalizationLanguageResource api_v3_localization_language_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LocalizationLanguageResource**](LocalizationLanguageResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

