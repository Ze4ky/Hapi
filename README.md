# Hapi

一个简单的api测试工具通过编写`JSON`来配置

```JSON
[
  {
    "name": "",
    "url": "",
    "method": "",
    "payload": {
      "headers": {},
      "body": {},
    }
  }
]
```

`name`对于请求本身没有影响会在运行hapi_cli时输出内容不限制<br/>
`url` Hapi会根据url的内容向url发送请求<br/>
`method` Hapi会根据method的内容决定请求方式 (目前只支持POST和GET)<br/>
`payload` 包含请求头和请求体
`payload.header` 请求头内容直接和http请求的内容对应如`Content-Type`<br/>
`payload.body` 根据api的实际情况决定不限制内容
