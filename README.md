# nrf-radio-gadget

## Python Protocol buffers compilation

```shell
mkdir -p client
protoc --proto_path=proto --python_out=client --pyi_out=client proto/ieee_802_15_4.proto 
```
