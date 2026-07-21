# Hapi

一个简单的api测试工具通过编写`JSON`来配置

# hapi_cli

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
`method` Hapi会根据method的内容决定请求方式<br/>
`payload` 包含请求头和请求体<br/>
`payload.header` 请求头内容直接和http请求的内容对应如`Content-Type`<br/>
`payload.body` 根据api的实际情况决定不限制内容

配置完成运行hapi_cli

```
hapi_cli -f /to/path/request.json
```

如果运行目录下有`request.json`

```
hapi_cli
```

> [!TIP]
> hapi_cli默认不完整显示请求的内容如果需要显示万丈的请求内容添加`--enable_complete_dispay参数`

# hapi_tui

基于ratatui编写的程序用于更直观的使用hapi

> [!WARNING]
> hapi_tui的运行目录下必须有request.json否则无法运行

在工作目录下运行

```
hapi_tui
```
