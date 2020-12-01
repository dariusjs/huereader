# huereader

The Huereader will discover local Hue Bridges in the network and scan for Hue Motion, Light and Temperature Sensors. These sensors can then be posted to influxdb.

#### To get an API key from the bridge
```
curl -d '{"devicetype:"application_name#device_name"}' --header "Content-Type: application/json" --request POST http://<bridge IP address>/api
```


#### Run the program
You will provide a Hue API Key as well as a InfluxDB address, port and database name. Huereader will then run the discovery and metric collection.

```
HUE_API_KEY=some_api_key cargo run  INFLUX_DB_ADDRESS=http://INFLUXDB_ADDRESS:8086/write?db=homestats huereader
```
