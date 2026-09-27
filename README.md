# Hapi

一个简单的api测试工具通过编写`JSON`来配置

# hapi_cli
一个简单的cli工具
```
Usage: hapi_cli <COMMAND>

Commands:
  init
  run
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

第一次使用需要在项目目录下运行初始化(也可以手动创建相关文件)

```
hapi init
```
它会在项目目录下创建
```
.hapi/
  |
  ----api.json //api接口配置文件
```


以下是`api.json`文件内容的示例
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

配置完成后如果要进行全面测试执行
```
hapi run all
```
它会把`api.json`下配置的所有接口都进行测试
如果只是想测试某个特定的接口可以输入它的编号(现在只能输入数字)
如要测试文件内配置的第1个接口则可以
```
hapi run 1
```
测试结果会保存在`.hapi_result`目录下保存的文件名格式为: 测试方式(all或者编号) + 测试时的时间戳

比如测试编号为1的接口保存的文件名为: `index_1_1790516809.json`
