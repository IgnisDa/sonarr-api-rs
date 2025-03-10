# \NamingConfigApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_naming_examples_get**](NamingConfigApi.md#api_v3_config_naming_examples_get) | **GET** /api/v3/config/naming/examples | 
[**api_v3_config_naming_get**](NamingConfigApi.md#api_v3_config_naming_get) | **GET** /api/v3/config/naming | 
[**api_v3_config_naming_id_get**](NamingConfigApi.md#api_v3_config_naming_id_get) | **GET** /api/v3/config/naming/{id} | 
[**api_v3_config_naming_id_put**](NamingConfigApi.md#api_v3_config_naming_id_put) | **PUT** /api/v3/config/naming/{id} | 



## api_v3_config_naming_examples_get

> api_v3_config_naming_examples_get(rename_episodes, replace_illegal_characters, colon_replacement_format, custom_colon_replacement_format, multi_episode_style, standard_episode_format, daily_episode_format, anime_episode_format, series_folder_format, season_folder_format, specials_folder_format, id, resource_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rename_episodes** | Option<**bool**> |  |  |
**replace_illegal_characters** | Option<**bool**> |  |  |
**colon_replacement_format** | Option<**i32**> |  |  |
**custom_colon_replacement_format** | Option<**String**> |  |  |
**multi_episode_style** | Option<**i32**> |  |  |
**standard_episode_format** | Option<**String**> |  |  |
**daily_episode_format** | Option<**String**> |  |  |
**anime_episode_format** | Option<**String**> |  |  |
**series_folder_format** | Option<**String**> |  |  |
**season_folder_format** | Option<**String**> |  |  |
**specials_folder_format** | Option<**String**> |  |  |
**id** | Option<**i32**> |  |  |
**resource_name** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_naming_get

> models::NamingConfigResource api_v3_config_naming_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_naming_id_get

> models::NamingConfigResource api_v3_config_naming_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_naming_id_put

> models::NamingConfigResource api_v3_config_naming_id_put(id, naming_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**naming_config_resource** | Option<[**NamingConfigResource**](NamingConfigResource.md)> |  |  |

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

