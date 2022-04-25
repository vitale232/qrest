# qrest

A small CLI tool to query [ArcGIS REST API](https://developers.arcgis.com/rest/services-reference/enterprise/query-feature-service-layer-.htm) services, implemented in Rust. The server response is returned as pretty JSON.

Said another way, Query REST, aka qrest. Pronounced crest ⛰️

Usage example:

```shell
$ > ./qrest https://gisservices.its.ny.gov/arcgis/rest/services/NYS_Place_Points/FeatureServer/0/query --where "County = 'Essex' AND PlaceType = 'Incorporated Town'" --count
  > "{\"count\":18}"
```

