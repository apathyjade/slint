# 签名

``` shell
codesign --force --deep --sign "Apple Development: apathyjade" ./Payload/MyApp.app  
```

## 打包

``` shell
zip -r MyApp.ipa Payload
```
