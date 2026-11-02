# Linker 软件综述

## System

### deviceUniqueID

- **类型**: 字符串 (String)
- **描述**: 设备的唯一标识符
- **示例**: `"0123456789abcdef0123456789abcdef"`

### deviceUniqueIDHash

- **类型**: 字符串 (String)
- **描述**: 设备唯一标识符的哈希值
- **编码**: `Base64`
- **示例**: `"ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI="`
- **Hash算法**: `Argon2id`
- **盐值**: 固定盐值 `ByteArray("iot-device-unique-id-salt")`
- **参数**:
    - `内存`: 8192 KB
    - `迭代次数`: 2
    - `并行度`: 1
    - `输出长度`: 32 字节

## MaintenanceTool

### 默认资源分发服务

- **URL格式**: `${IFW_REPOSITORY_BASE_URL}/${PROJECT_NAME}/repository/${IFW_PLATFORM}-${IFW_ARCH}`
- **示例**:
    - Linux-x86_64平台: `https://repository.example.com/linker/repository/linux-x86_64`
    - Android-arm64平台: `https://repository.example.com/linker/repository/android-aarch64`

## MQTT 客户端

- **MQTT 版本**: 5.0
- **连接加密**: TLS 1.2 或更高版本
- **认证方式**: 用户名和密码认证
- **保持连接**: 启用，默认保持连接间隔为 30 秒
- **清理会话**: 禁用，保持会话状态
- **遗嘱主题**: `{topicPrefix}/will`
- **遗嘱消息**: `"Client disconnected unexpectedly"`
- **遗嘱 QoS**: 1
- **遗嘱保留**: 否

### userProperties

- **类型**: 字典 (Dictionary)
- **描述**: 包含 MQTT 客户端的用户属性
- **示例**:

```json
{
  "kernelType": "linux",
  "platformType": "ubuntu",
  "device_unique_id_hash": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI=",
  "client_name": "Linker-Client-001",
  "application_name": "Linker",
  "application_version": "1.0.0"
}
```

- **用途**: 用于在 MQTT 连接中传递设备和客户端的相关信息

### clientName

- **类型**: 字符串 (String)
- **描述**: MQTT 客户端的名称
- **示例**: `"Linker-Client-001"`
- **用途**: 用于标识 MQTT 客户端，同一设备上具有唯一性

### topicPrefix

- **类型**: 字符串 (String)
- **描述**: MQTT 主题的前缀，由 `deviceUniqueIDHash` 和 `clientName` 的百分号编码组合而成
- **示例**: `"ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI%3D/ReservedClient"`
- **用途**: 用于组织和分类 MQTT 消息主题，由客户端自动生成

### topicSuffix

- **类型**: 字符串 (String)
- **描述**: MQTT 主题的后缀，用于进一步细分主题
- **示例**: `"protocol_stack/Control/ReservedControlProtocolStack/system/request"`
- **用途**: 用于指定具体的消息类型或功能，由使用客户端者定义，层级字符串使用百分号编码

### 保留客户端

#### ReservedClient

- **clientName**: `"ReservedClient"`
- **username**: `${deviceUniqueIDHash}`
- **password**: `${deviceUniqueID}`
- **hostname**: `mqtt.example.com`
- **port**: `8883`
- **isSSL**: `true`
- **描述**: 保留的 MQTT 客户端，用于控制协议栈的关联，不可修改

## 协议栈

### protocolStackName

- **类型**: 字符串 (String)
- **描述**: 协议栈的名称
- **示例**: `"Control"`
- **用途**: 用于标识不同的协议栈，同一设备上具有唯一性

### protocolStackType

- **类型**: enum class ProtocolType
- **描述**: 协议栈的类型
- **枚举值**:
    - `Control`
    - `DTU`
    - `JB3208`
    - `NFS23030`

### outputFormat

- **类型**: enum class DataFormat
- **描述**: 协议栈输出的数据格式
- **枚举值**:
    - `Raw`
    - `JSON`
    - `GB26875`

### 控制协议栈(By MQTT)

- **描述**: 通过 MQTT 对设备进行控制和管理的协议栈
- **mqttSubscriptionConfigs**:
    - **主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/+/request`
    - **QoS**: 2
    - **描述**: 用于接收控制协议栈的请求消息，+号通配符表示可以匹配任意模块(action)

#### 请求消息格式

- **action**: 请求的模块名称
- **message_type**: 消息类型，固定为 `request`
- **method**: 请求的方法名称
- **source**: 消息的来源，由请求方指定
- **target**: 消息的目标，目标的`${deviceUniqueIDHash}`
- **timestamp**: 请求的时间戳，单位为毫秒
- **transaction_id**: 事务 ID，用于匹配请求和响应，由请求方指定，int64
- **version**: 协议版本
- **timeout**: 请求的超时时间，单位为毫秒
- **params**: 请求的参数，具体内容根据方法而定
- **示例**:

```json
[
  {
    "action": "system",
    "message_type": "request",
    "method": "deviceUniqueID",
    "source": "control",
    "target": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI=",
    "timestamp": 1767770515152,
    "transaction_id": 1,
    "version": "1.0.0",
    "timeout": 3000
  },
  {
    "params": ""
  }
]
```

#### 响应消息格式

- **action**: 响应的模块名称
- **is_error**: 是否为错误响应，布尔值
- **message_type**: 消息类型，固定为 `response`
- **method**: 响应的方法名称
- **source**: 消息的来源，deviceUniqueIDHash
- **target**: 消息的目标，由请求方提供
- **timestamp**: 响应的时间戳，单位为毫秒
- **transaction_id**: 事务 ID，用于匹配请求和响应，由请求方提供，int64
- **version**: 协议版本
- **result**: 响应的结果，具体内容根据方法而定
- **示例**:

```json
[
  {
    "action": "system",
    "is_error": false,
    "message_type": "response",
    "method": "deviceUniqueID",
    "source": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI=",
    "target": "control",
    "timestamp": 1768196104786,
    "transaction_id": 1,
    "version": "1.0.0"
  },
  {
    "result": "0123456789abcdef0123456789abcdef"
  }
]
```

```json
[
  {
    "action": "system",
    "is_error": true,
    "message_type": "response",
    "method": "deviceUniqueID",
    "source": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI=",
    "target": "control",
    "timestamp": 1768197215375,
    "transaction_id": 1,
    "version": "1.0.0"
  },
  {
    "error": "error_message_here"
  }
]
```

#### system 模块

- **action**: `system`
- **请求主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/system/request`
- **响应主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/system/response`

- ***methods:***

- `deviceUniqueID`: 获取设备唯一标识符
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "deviceUniqueID"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": "0123456789abcdef0123456789abcdef"
            }
          ]
         ```

- `deviceUniqueIDHash`: 获取设备唯一标识符的哈希值
    - 请求方法和参数示例(不含通用字段):

         ```json
            [
              {
                "method": "deviceUniqueIDHash"
              },
              {
                "params": ""
              }
            ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
           [
             {
               "result": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI="
             }
           ]
         ```
- `info`: 获取设备信息
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "info"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": {
                "bootUniqueId": "cf498b3a-675e-4784-ad1e-cbbc780e2213",
                "buildAbi": "arm64-little_endian-lp64",
                "buildCpuArchitecture": "arm64",
                "currentCpuArchitecture": "arm64",
                "kernelType": "linux",
                "kernelVersion": "4.19.232",
                "machineHostName": "localhost",
                "machineUniqueId": "",
                "platformType": "android",
                "platformVersion": "12.0",
                "prettyPlatformName": "Android 12L (12.0)"
              }
            }
          ]
         ```

- `changePassword`: 更改设备密码
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "changePassword"
            },
            {
              "params": {
                "currentPassword": "current_password_here",
                "newPassword": "new_password_here"
              }
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
          {

            "result": "$argon2id$v=19$m=8192,t=2,p=1$example-argon2-hash"
          }
          ]
         ```

- `performReboot`: 重启设备
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "performReboot"
            },
            {
              "params": ""
            }
          ]
         ```

        - 不会有成功响应结果，因为设备会重启

- `iccIDs`: 获取设备的 ICCID 列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "iccIDs"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "8986002212345678901",
                "8986002212345678902"
              ]
            }
          ]
         ```

- `setTime`: 设置设备时间
    - 请求方法和参数示例(不含通用字段):
        - `params`: 时间戳，单位为毫秒，自 Unix 纪元以来的毫秒数

             ```json
              [
                {
                  "method": "setTime"
                },
                {
                  "params": 16767770515152
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": 16767770515152
            }
          ]
         ```

- `setTimeZone`: 设置设备时区
    - 请求方法和参数示例(不含通用字段):
        - `params`: 时区字符串

             ```json
              [
                {
                  "method": "setTimeZone"
                },
                {
                  "params": "Asia/Shanghai"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": "Asia/Shanghai"
            }
          ]
         ```

- `availableTimeZoneIDs`: 获取可用的时区 ID 列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableTimeZoneIDs"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
          {
            "result": [
              "America/New_York",
              "Europe/London",
              "Asia/Shanghai"
            ]
          }
          ]
         ```

- `systemTimeZoneID`: 获取当前系统时区 ID
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "systemTimeZoneID"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

        ```json
             [
               {
                 "result": "Asia/Shanghai"
               }
             ]
       ```

- `applicationInfo`: 获取应用程序信息
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "applicationInfo"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
          {
            "result": {
              "applicationName": "Linker",
              "applicationVersion": "1.0.0",
              "organizationName": "Example-IoT",
              "organizationDomain": "example.com"
            }
          }
          ]
         ```

#### serial_port_manager 模块

- **action**: `serial_port_manager`
- **请求主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/serial_port_manager/request`
- **响应主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/serial_port_manager/response`

- ***methods:***

- `availableSerialPorts`: 获取可用的串口列表
    - 请求方法和参数示例(不含通用字段):

         ```json
            [
              {
                "method": "availableSerialPorts"
              },
              {
                "params": ""
              }
            ]
         ```

    - 成功响应结果示例(不含通用字段):
        - `attachedProtocols`: 关联的协议栈的名称的列表

             ```json
               [
                 {
                 "result": [
                    {
                     "attachedProtocols": [],
                     "baudRate": 115200,
                     "dataBits": "Data8",
                     "flowControl": "NoFlowControl",
                     "isDataCarrierDetect": false,
                     "isOpen": false,
                     "parity": "NoParity",
                     "portName": "ttyS1",
                     "stopBits": "OneStop"
                   },
                   {
                     "attachedProtocols": [],
                     "baudRate": 115200,
                     "dataBits": "Data8",
                     "flowControl": "NoFlowControl",
                     "isDataCarrierDetect": false,
                     "isOpen": false,
                     "parity": "NoParity",
                     "portName": "ttyS10",
                     "stopBits": "OneStop"
                   }
                  ]
                 }
              ]
             ```

- `availableDataBits`: 获取支持的数据位选项
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableDataBits"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "Data5",
                "Data6",
                "Data7",
                "Data8"
              ]
            }
          ]
         ```

- `availableParities`: 获取支持的校验位选项
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableParities"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "NoParity",
                "EvenParity",
                "OddParity",
                "MarkParity",
                "SpaceParity"
              ]
            }
          ]
         ```

- `availableStopBits`: 获取支持的停止位选项
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {          
              "method": "availableStopBits"
            },
            {         
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "OneStop",
                "OneAndHalfStop",
                "TwoStop"
              ]
            }
          ]
         ```

- `availableFlowControls`: 获取支持的流控选项
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableFlowControls"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "NoFlowControl",
                "HardwareControl",
                "SoftwareControl"
              ]
            }
          ]
         ```

- `openSerialPort`: 打开指定的串口
    - 请求方法和参数示例(不含通用字段):
        - `params`: 串口名称

             ```json
              [
                {
                  "method": "openSerialPort"
                },
                {
                  "params": "ttyS1"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": true
            }
          ]
         ```

- `closeSerialPort`: 关闭指定的串口
    - 请求方法和参数示例(不含通用字段):
        - `params`: 串口名称

             ```json
              [
                {
                  "method": "closeSerialPort"
                },
                {
                  "params": "ttyS1"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": false
            }
          ]
         ```

- `setBaudRate`: 设置指定串口的波特率
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含串口名称和波特率的对象
        - `portName`: 串口名称，字符串
        - `baudRate`: 波特率，int32

             ```json
              [
                {
                  "method": "setBaudRate"
                },
                {
                  "params": {
                    "portName": "ttyS1",
                    "baudRate": 9600
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": 9600
            }
          ]
         ```

- `setDataBits`: 设置指定串口的数据位
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含串口名称和数据位的对象
        - `portName`: 串口名称，字符串
        - `dataBits`: 数据位，枚举值之一

             ```json
              [
                {
                  "method": "setDataBits"
                },
                {
                  "params": {
                    "portName": "ttyS1",
                    "dataBits": "Data7"
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": "Data7"
            }
          ]
         ```

- `setParity`: 设置指定串口的校验位
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含串口名称和校验位的对象
        - `portName`: 串口名称，字符串
        - `parity`: 校验位，枚举值之一

             ```json
              [
                {
                  "method": "setParity"
                },
                {
                  "params": {
                    "portName": "ttyS1",
                    "parity": "EvenParity"
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": "EvenParity"
            }
          ]
         ```

- `setStopBits`: 设置指定串口的停止位
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含串口名称和停止位的对象
        - `portName`: 串口名称，字符串
        - `stopBits`: 停止位，枚举值之一

             ```json
              [
                {
                  "method": "setStopBits"
                },
                {
                  "params": {
                    "portName": "ttyS1",
                    "stopBits": "TwoStop"
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
            [
                {
                "result": "TwoStop"
                }
            ]
         ```

- `setFlowControl`: 设置指定串口的流控
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含串口名称和流控的对象
        - `portName`: 串口名称，字符串
        - `flowControl`: 流控，枚举值之一

             ```json
              [
                {
                  "method": "setFlowControl"
                },
                {
                  "params": {
                    "portName": "ttyS1",
                    "flowControl": "HardwareControl"
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": "HardwareControl"
            }
          ]
         ```

#### logger 模块

- **action**: `logger`
- **请求主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/logger/request`
- **响应主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/logger/response`
- **日志消息发布主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/logger/new_log_message`
    - **消息格式**:
        - **type**: 日志级别，枚举值之一
        - **message**: 日志消息内容，字符串
        - **示例**:

             ```json
                {
                  "type": "QtWarningMsg",
                  "message": "This is a warning message."
                }
             ```

- ***methods:***

- `setMaxLogsFileSize`: 设置日志文件区的最大大小
    - 请求方法和参数示例(不含通用字段):
        - `params`: 最大日志文件大小，单位为字节，int32

             ```json
              [
                {
                  "method": "setMaxLogsFileSize"
                },
                {
                  "params": 10485760
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": 10485760
            }
          ]
         ```

- `maxLogsFileSize`: 获取日志文件的最大大小
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "maxLogsFileSize"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": 10485760
            }
          ]
         ```

- `setGlobalLogLevel`: 设置全局日志级别
    - 请求方法和参数示例(不含通用字段):
        - `params`: 日志级别，枚举值之一

             ```json
              [
                {
                  "method": "setGlobalLogLevel"
                },
                {
                  "params": "QtWarningMsg"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": "QtWarningMsg"
            }
          ]
         ```

- `globalLogLevel`: 获取全局日志级别
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "globalLogLevel"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": "QtWarningMsg"
            }
          ]
         ```

- `availableLogLevels`: 获取支持的日志级别列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableLogLevels"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "QtDebugMsg",
                "QtInfoMsg",
                "QtWarningMsg",
                "QtCriticalMsg",
                "QtFatalMsg"
              ]
            }
          ]
         ```

- `openLogChannel`: 打开日志通道，在 `new_log_message` 通道发布日志消息
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "openLogChannel"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `closeLogChannel`: 关闭日志通道，停止在 `new_log_message` 通道发布日志消息
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "closeLogChannel"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

#### maintenance_tool 模块

- **action**: `maintenance_tool`
- **请求主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/maintenance_tool/request`
- **响应主题**: `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/maintenance_tool/response`
- **检查更新完成消息发布主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/maintenance_tool/check_updates/finished`
    - **消息格式（Linux平台）**:
        - **exitCode**: 退出代码，int
        - **exitStatus**: 退出状态，字符串
        - **processError**: 进程错误，字符串
        - **standardOutput**: 标准输出内容，字符串
        - **standardError**: 标准错误内容，字符串
        - **updates**: 可用更新列表，数组
        - **tempRepositoryURI**: 临时软件仓库的 URL，字符串
        - **示例**:

             ```json
                {
                  "exitCode": 0,
                  "exitStatus": "NormalExit",
                  "processError": "",
                  "standardOutput": "Update check completed successfully.",
                  "standardError": "",
                  "updates": [
                    {
                      "name": "example-package",
                      "version": "1.2.3",
                      "size": 2048,
                      "id": "example-package-1.2.3"
                    }
                  ],
                  "tempRepositoryURI": "https://example.com/temp-repo"
                }
             ```
    - **消息格式（Android平台）**:
        - **networkError**: http 网络错误
        - **data**: 响应数据，字符串
        - **errorString**: 错误描述，字符串
        - **updates**: 可用更新列表，数组
        - **tempRepositoryURI**: 临时软件仓库的 URL，字符串
        - **示例**:

             ```json
                {
                  "networkError": "NoError",
                  "data": "{...}",
                  "errorString": "",
                  "updates": [
                    {
                      "name": "example-package",
                      "version": "1.2.3",
                      "size": 2048,
                      "id": "example-package-1.2.3"
                    }
                  ],
                  "tempRepositoryURI": "https://example.com/temp-repo"
                }
             ```

- **更新完成消息发布主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/maintenance_tool/update/finished`
    - **消息格式**:
        - **exitCode**: 退出代码，int
        - **exitStatus**: 退出状态，字符串
        - **processError**: 进程错误，字符串
        - **standardOutput**: 标准输出内容，字符串
        - **standardError**: 标准错误内容，字符串
        - **tempRepositoryURI**: 临时软件仓库的 URL，字符串
        - **示例**:

             ```json
              {
                "exitCode": 0,
                "exitStatus": "NormalExit",
                "processError": "",
                "standardOutput": "Update completed successfully.",
                "standardError": "",
                "tempRepositoryURI": "https://example.com/temp-repo"
              }
             ```

- **更新标准输出消息发布主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/maintenance_tool/update/standard_output`
    - **消息格式**: 标准输出内容，字符串
        - **示例**:

             ```text
                Downloading package example-package...
             ```

- ***methods:***

- `addRepository`: 添加软件仓库
    - 请求方法和参数示例(不含通用字段):
        - `params`: 软件仓库的 URL，字符串

             ```json
              [
                {
                  "method": "addRepository"
                },
                {
                  "params": "https://example.com/repo"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `addTempRepository`: 添加临时软件仓库
    - 请求方法和参数示例(不含通用字段):
        - `params`: 临时软件仓库的 URL，字符串

             ```json
              [
                {
                  "method": "addTempRepository"
                },
                {
                  "params": "https://example.com/temp-repo"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `checkUpdates`: 检查软件更新，异步方法，通过 `check_updates/finished` 通道发布检查结果
    - 请求方法和参数示例(不含通用字段):
        - `params`: 可空，软件仓库的 URL，字符串，只在本次检查中使用

             ```json
              [
                {
                  "method": "checkUpdates"
                },
                {
                  "params": "https://example.com/repo"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `update`: 进行软件更新，异步方法，通过 `update/finished` 通道发布更新结果，通过 `update/standard_output` 发布更新过程中的标准输出
    - 请求方法和参数示例(不含通用字段):
        - `params`: 可空，软件仓库的 URL，字符串，只在本次更新中使用

             ```json
              [
                {
                  "method": "update"
                },
                {
                  "params": "https://example.com/repo"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

#### network_interface_manager 模块

- **action**: `network_interface_manager`
- **请求主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/network_interface_manager/request`
- **响应主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/network_interface_manager/response`

- ***methods:***

- `allInterfaces`: 获取所有网络接口的信息
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "allInterfaces"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
              {
                "result": [
                  {
                    "addressEntries": [],
                    "dhcpClient": {
                      "leaseTimeIPv4": 0,
                      "leaseTimeIPv6": 0,
                      "serverIPv4": "",
                      "serverIPv6": ""
                    },
                    "hardwareAddress": "10:FF:E0:C5:E1:4B",
                    "humanReadableName": "enp7s0",
                    "index": 2,
                    "isEnabledDHCPClient": false,
                    "isRunning": false,
                    "isUp": true,
                    "maximumTransmissionUnit": 1500,
                    "name": "enp7s0",
                    "type": "Ethernet"
                  },
                  {
                    "addressEntries": [
                      {
                        "broadcast": "10.0.0.255",
                        "ip": "10.0.0.10",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24
                      },
                      {
                        "broadcast": "",
                        "ip": "fe80::be4c:83a0:690b:f622%wlp6s0",
                        "netmask": "ffff:ffff:ffff:ffff::",
                        "prefixLength": 64
                      }
                    ],
                    "dhcpClient": {
                      "leaseTimeIPv4": 0,
                      "leaseTimeIPv6": 0,
                      "serverIPv4": "",
                      "serverIPv6": ""
                    },
                    "hardwareAddress": "58:10:31:E9:E5:A3",
                    "humanReadableName": "wlp6s0",
                    "index": 3,
                    "isEnabledDHCPClient": false,
                    "isRunning": true,
                    "isUp": true,
                    "maximumTransmissionUnit": 1500,
                    "name": "wlp6s0",
                    "type": "Wifi"
                  }
                ]
              }
          ]
         ```

- `addInterfaceAddressEntry`: 为指定网络接口添加地址条目（不适用于Android平台）
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含接口名称和地址条目的对象
        - `interfaceName`: 网络接口名称，字符串
        - `entry`: 地址条目对象
            - `ip`: IP 地址，字符串
            - `netmask`: 子网掩码，字符串
            - `prefixLength`: 前缀长度，int32
            - `broadcast`: 广播地址，字符串

                ```json
                [
                  {
                    "method": "addInterfaceAddressEntry"
                  },
                  {
                    "params": {
                      "interfaceName": "wlp6s0",
                      "entry": {
                        "ip": "10.0.1.1",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24,
                        "broadcast": "10.0.1.255"
                      }
                    }
                  }
                ]
                ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
              {
                "result": [
                  {
                    "addressEntries": [
                      {
                        "broadcast": "10.0.0.255",
                        "ip": "10.0.0.10",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24
                      },
                      {
                        "broadcast": "",
                        "ip": "fe80::be4c:83a0:690b:f622%wlp6s0",
                        "netmask": "ffff:ffff:ffff:ffff::",
                        "prefixLength": 64
                      }
                    ],
                    "dhcpClient": {
                      "leaseTimeIPv4": 0,
                      "leaseTimeIPv6": 0,
                      "serverIPv4": "",
                      "serverIPv6": ""
                    },
                    "hardwareAddress": "58:10:31:E9:E5:A3",
                    "humanReadableName": "wlp6s0",
                    "index": 3,
                    "isEnabledDHCPClient": false,
                    "isRunning": true,
                    "isUp": true,
                    "maximumTransmissionUnit": 1500,
                    "name": "wlp6s0",
                    "type": "Wifi"
                  }
                ]
              }
          ]
         ```

- `removeInterfaceAddressEntry`: 从指定网络接口移除地址条目（不适用于Android平台）
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含接口名称和地址条目的对象
        - `interfaceName`: 网络接口名称，字符串
        - `entry`: 地址条目对象
            - `ip`: IP 地址，字符串
            - `netmask`: 子网掩码，字符串
            - `prefixLength`: 前缀长度，int32
            - `broadcast`: 广播地址，字符串

                ```json
                [
                  {
                    "method": "removeInterfaceAddressEntry"
                  },
                  {
                    "params": {
                      "interfaceName": "wlp6s0",
                      "entry": {
                        "ip": "10.0.1.1",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24,
                        "broadcast": "10.0.1.255"
                      }
                    }
                  }
                ]
                ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
              {
                "result": [
                  {
                    "addressEntries": [
                      {
                        "broadcast": "10.0.0.255",
                        "ip": "10.0.0.10",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24
                      },
                      {
                        "broadcast": "",
                        "ip": "fe80::be4c:83a0:690b:f622%wlp6s0",
                        "netmask": "ffff:ffff:ffff:ffff::",
                        "prefixLength": 64
                      }
                    ],
                    "dhcpClient": {
                      "leaseTimeIPv4": 0,
                      "leaseTimeIPv6": 0,
                      "serverIPv4": "",
                      "serverIPv6": ""
                    },
                    "hardwareAddress": "58:10:31:E9:E5:A3",
                    "humanReadableName": "wlp6s0",
                    "index": 3,
                    "isEnabledDHCPClient": false,
                    "isRunning": true,
                    "isUp": true,
                    "maximumTransmissionUnit": 1500,
                    "name": "wlp6s0",
                    "type": "Wifi"
                  }
                ]
              }
          ]
         ```

- `setInterfaceAddressEntry`: 设置指定网络接口的地址条目
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含接口名称、旧地址条目和新地址条目的对象
        - `interfaceName`: 网络接口名称，字符串
        - `oldEntry`: 旧地址条目对象（Android平台不需要此参数）
            - `ip`: IP 地址，字符串
            - `netmask`: 子网掩码，字符串
            - `prefixLength`: 前缀长度，int32
            - `broadcast`: 广播地址，字符串
        - `newEntry`: 新地址条目对象
            - `ip`: IP 地址，字符串
            - `netmask`: 子网掩码，字符串
            - `prefixLength`: 前缀长度，int32
            - `broadcast`: 广播地址，字符串

                ```json
                [
                  {
                    "method": "setInterfaceAddressEntry"
                  },
                  {
                    "params": {
                      "interfaceName": "wlp6s0",
                      "oldEntry": {
                        "ip": "10.0.1.1",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24,
                        "broadcast": "10.0.1.255"
                      },
                      "newEntry": {
                        "ip": "10.0.1.1",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24,
                        "broadcast": "10.0.1.255"
                      }
                    }
                  }
                ]
                ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
              {
                "result": [
                  {
                    "addressEntries": [
                      {
                        "broadcast": "10.0.0.255",
                        "ip": "10.0.0.10",
                        "netmask": "255.255.255.0",
                        "prefixLength": 24
                      },
                      {
                        "broadcast": "",
                        "ip": "fe80::be4c:83a0:690b:f622%wlp6s0",
                        "netmask": "ffff:ffff:ffff:ffff::",
                        "prefixLength": 64
                      }
                    ],
                    "dhcpClient": {
                      "leaseTimeIPv4": 0,
                      "leaseTimeIPv6": 0,
                      "serverIPv4": "",
                      "serverIPv6": ""
                    },
                    "hardwareAddress": "58:10:31:E9:E5:A3",
                    "humanReadableName": "wlp6s0",
                    "index": 3,
                    "isEnabledDHCPClient": false,
                    "isRunning": true,
                    "isUp": true,
                    "maximumTransmissionUnit": 1500,
                    "name": "wlp6s0",
                    "type": "Wifi"
                  }
                ]
              }
          ]
         ```

- `setInterfaceUp`: 将指定网络接口设置为启用状态
    - 请求方法和参数示例(不含通用字段):
        - `params`: 网络接口名称，字符串

             ```json
              [
                {
                  "method": "setInterfaceUp"
                },
                {
                  "params": "wlp6s0"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": true
            }
          ]
         ```

- `setInterfaceDown`: 将指定网络接口设置为禁用状态
    - 请求方法和参数示例(不含通用字段):
        - `params`: 网络接口名称，字符串

             ```json
              [
                {
                  "method": "setInterfaceDown"
                },
                {
                  "params": "wlp6s0"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": false
            }
          ]
         ```

- `enableInterfaceDHCPClient`: 启用指定网络接口的 DHCP 客户端
    - 请求方法和参数示例(不含通用字段):
        - `params`: 网络接口名称，字符串

             ```json
              [
                {
                  "method": "enableInterfaceDHCPClient"
                },
                {
                  "params": "wlp6s0"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": true
            }
          ]
         ```

- `disableInterfaceDHCPClient`: 禁用指定网络接口的 DHCP 客户端
    - 请求方法和参数示例(不含通用字段):
        - `params`: 网络接口名称，字符串

             ```json
              [
                {
                  "method": "disableInterfaceDHCPClient"
                },
                {
                  "params": "wlp6s0"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": false
            }
          ]
         ```

#### tcp_client_manager 模块

- **action**: `tcp_client_manager`
- **请求主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/tcp_client_manager/request`
- **响应主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/tcp_client_manager/response`

- ***methods:***

- `availableTCPClients`: 获取可用的 TCP 客户端列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableTCPClients"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
            [
              {
                "result": [
                  {
                    "attachedProtocols": [],
                    "clientName": "test-tcp-2",
                    "hostName": "127.0.0.1",
                    "localAddress": "",
                    "localPort": 0,
                    "peerAddress": "",
                    "peerName": "127.0.0.1",
                    "peerPort": 0,
                    "port": 12345,
                    "state": "UnconnectedState"
                  }
                ]
              }
          ]
         ```

- `registerTCPClient`: 注册新的 TCP 客户端
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含客户端名称、主机名和端口的对象
        - `clientName`: TCP 客户端名称，字符串
        - `hostName`: 目标主机名或 IP 地址，字符串
        - `port`: 目标端口号，uint16

             ```json
              [
                {
                  "method": "registerTCPClient"
                },
                {
                  "params": {
                    "clientName": "test-tcp-2",
                    "hostName": "127.0.0.1",
                    "port": 12345
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
         [
           {
             "result": {
               "attachedProtocols": [],
               "clientName": "test-tcp-2",
               "hostName": "127.0.0.1",
               "localAddress": "",
               "localPort": 0,
               "peerAddress": "",
               "peerName": "",
               "peerPort": 0,
               "port": 12345,
               "state": "UnconnectedState"
             }
           }
         ]
         ```

- `unregisterTCPClient`: 注销指定的 TCP 客户端
    - 请求方法和参数示例(不含通用字段):
        - `params`: TCP 客户端名称，字符串

             ```json
              [
                {
                  "method": "unregisterTCPClient"
                },
                {
                  "params": "test-tcp-2"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `setTCPClientAndReconnect`: 设置指定 TCP 客户端的主机名和端口，并重新连接
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含客户端名称、主机名和端口的对象
        - `clientName`: TCP 客户端名称，字符串
        - `hostName`: 目标主机名或 IP 地址，字符串
        - `port`: 目标端口号，int32

             ```json
              [
                {
                  "method": "setTCPClientAndReconnect"
                },
                {
                  "params": {
                    "clientName": "test-tcp-2",
                    "hostName": "127.0.0.1",
                    "port": 12345
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
         [
           {
             "result": {
               "attachedProtocols": [],
               "clientName": "test-tcp-2",
               "hostName": "127.0.0.1",
               "localAddress": "",
               "localPort": 0,
               "peerAddress": "",
               "peerName": "",
               "peerPort": 0,
               "port": 12345,
               "state": "UnconnectedState"
             }
           }
         ]
         ```

#### mqtt_client_manager 模块

- **action**: `mqtt_client_manager`
- **请求主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/mqtt_client_manager/request`
- **响应主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/mqtt_client_manager/response`

- ***methods:***

- `availableMQTTClients`: 获取可用的 MQTT 客户端列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableMQTTClients"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
            [
              {
                "result": [
                  {
                    "attachedProtocols": [
                       "ReservedControlProtocolStack"
                    ],
                    "autoKeepAlive": true,
                    "cleanSession": false,
                    "clientId": "05b53a000363410794f4268",
                    "clientName": "ReservedClient",
                    "hostname": "localhost",
                    "isSSL": false,
                    "keepAlive": 30,
                    "password": "0123456789abcdef0123456789abcdef",
                    "port": 1883,
                    "protocolVersion": "MQTT_5_0",
                    "registeredSubscriptionConfigs": [
                      {
                        "properties": {},
                        "qos": 2,
                        "topicSuffix": "protocol_stack/Control/ReservedControlProtocolStack/+/request",
                        "unsubscriptionProperties": {}
                      }
                    ],
                    "state": "Connected",
                    "subscriptions": [
                      {
                        "isSharedSubscription": false,
                        "qos": 2,
                        "reason": "",
                        "reasonCode": 2,
                        "sharedSubscriptionName": "",
                        "topicFilter": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI%3D/ReservedClient/protocol_stack/Control/ReservedControlProtocolStack/+/request",
                        "userProperties": []
                      }
                    ],
                    "username": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI=",
                    "willMessage": "Client disconnected unexpectedly",
                    "willQoS": 1,
                    "willRetain": true,
                    "willTopic": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI%3D/ReservedClient/will"
                  }
                ]
              }
          ]
         ```

- `registerMQTTClient`: 注册新的 MQTT 客户端
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含客户端名称、主机名和端口的对象
        - `clientName`: MQTT 客户端名称，字符串
        - `username`: MQTT 用户名，字符串
        - `password`: MQTT 密码，字符串
        - `hostname`: 目标主机名或 IP 地址，字符串
        - `port`: 目标端口号，uint16

             ```json
                [
                  {
                    "method": "registerMQTTClient"
                  },
                  {
                    "params": {
                      "clientName": "example-mqtt-client",
                      "username": "example-user",
                      "password": "example-pass",
                      "hostname": "mqtt.example.com",
                      "port": 1883
                    }
                  }
                ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
         [
           {
             "result": {
               "attachedProtocols": [],
               "autoKeepAlive": true,
               "cleanSession": false,
               "clientId": "291e854d39b54878befbdcc",
               "clientName": "example-mqtt-client",
               "hostname": "mqtt.example.com",
               "isSSL": true,
               "keepAlive": 30,
               "password": "example-pass",
               "port": 1883,
               "protocolVersion": "MQTT_5_0",
               "registeredSubscriptionConfigs": [],
               "state": "Disconnected",
               "subscriptions": [],
               "username": "example-user",
               "willMessage": "Client disconnected unexpectedly",
               "willQoS": 1,
               "willRetain": false,
               "willTopic": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI%3D/example-mqtt-client/will"
             }
           }
         ]
         ```

- `unregisterMQTTClient`: 注销指定的 MQTT 客户端

    - 请求方法和参数示例(不含通用字段):
        - `params`: MQTT 客户端名称，字符串

             ```json
              [
                {
                  "method": "unregisterMQTTClient"
                },
                {
                  "params": "example-mqtt-client"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `setIsSSLAndReconnect`: 设置指定 MQTT 客户端的 SSL 选项并重新连接
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含客户端名称和 SSL 选项的对象
        - `clientName`: MQTT 客户端名称，字符串
        - `isSSL`: 是否启用 SSL，布尔值

             ```json
              [
                {
                  "method": "setIsSSLAndReconnect"
                },
                {
                  "params": {
                    "clientName": "example-mqtt-client",
                    "isSSL": true
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": true
            }
          ]
         ```

- `setMQTTClientAndReconnect`: 设置指定 MQTT 客户端的连接参数并重新连接
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含客户端名称、主机名和端口的对象
        - `clientName`: MQTT 客户端名称，字符串
        - `username`: MQTT 用户名，字符串
        - `password`: MQTT 密码，字符串
        - `hostname`: 目标主机名或 IP 地址，字符串
        - `port`: 目标端口号，uint16

             ```json
              [
                {
                  "method": "setMQTTClientAndReconnect"
                },
                {
                  "params": {
                    "clientName": "example-mqtt-client",
                    "username": "example-user",
                    "password": "example-pass",
                    "hostname": "mqtt.example.com",
                    "port": 1883
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
         [
           {
             "result": {
               "attachedProtocols": [],
               "autoKeepAlive": true,
               "cleanSession": false,
               "clientId": "291e854d39b54878befbdcc",
               "clientName": "example-mqtt-client",
               "hostname": "mqtt.example.com",
               "isSSL": true,
               "keepAlive": 30,
               "password": "example-pass",
               "port": 1883,
               "protocolVersion": "MQTT_5_0",
               "registeredSubscriptionConfigs": [],
               "state": "Disconnected",
               "subscriptions": [],
               "username": "example-user",
               "willMessage": "Client disconnected unexpectedly",
               "willQoS": 1,
               "willRetain": false,
               "willTopic": "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI%3D/example-mqtt-client/will"
             }
           }
         ]
         ```

#### ntp_client_manager 模块

- **action**: `ntp_client_manager`
- **请求主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/ntp_client_manager/request`
- **响应主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/ntp_client_manager/response`

- ***methods:***

- `availableNTPClients`: 获取可用的 NTP 客户端列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableNTPClients"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
            [
              {
                "result": [
                  {
                    "clientName": "ali",
                    "hostName": "ntp.aliyun.com",
                    "port": 123
                  }
                ]
              }
            ]
         ```

- `sendNTPRequestsToAll`: 向所有注册的 NTP 服务器发送时间同步请求
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "sendNTPRequestsToAll"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `registerNTPClient`: 注册新的 NTP 客户端
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含客户端名称、主机名和端口的对象
        - `clientName`: NTP 客户端名称，字符串
        - `hostName`: 目标主机名或 IP 地址，字符串
        - `port`: 目标端口号，uint16

             ```json
              [
                {
                  "method": "registerNTPClient"
                },
                {
                  "params": {
                    "clientName": "example-ntp-client",
                    "hostName": "ntp.example.com",
                    "port": 123
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
         [
           {
             "result": {
               "clientName": "example-ntp-client",
               "hostName": "ntp.example.com",
               "port": 123
             }
           }
         ]
         ```

- `unregisterNTPClient`: 注销指定的 NTP 客户端
    - 请求方法和参数示例(不含通用字段):
        - `params`: NTP 客户端名称，字符串

             ```json
              [
                {
                  "method": "unregisterNTPClient"
                },
                {
                  "params": "example-ntp-client"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `setNTPClient`: 设置指定 NTP 客户端的连接参数
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含客户端名称、主机名和端口的对象
        - `clientName`: NTP 客户端名称，字符串
        - `hostName`: 目标主机名或 IP 地址，字符串
        - `port`: 目标端口号，uint16

             ```json
              [
                {
                  "method": "setNTPClient"
                },
                {
                  "params": {
                    "clientName": "example-ntp-client",
                    "hostName": "ntp.example.com",
                    "port": 123
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
         [
           {
             "result": {
               "clientName": "example-ntp-client",
               "hostName": "ntp.example.com",
               "port": 123
             }
           }
         ]
         ```

#### protocol_stack_manager 模块

- **action**: `protocol_stack_manager`
- **请求主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/protocol_stack_manager/request`
- **响应主题**:
  `{topicPrefix}/protocol_stack/${protocolStackType}/${protocolStackName}/protocol_stack_manager/response`

***methods:***

- `availableProtocolStacks`: 获取可用的协议栈列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableProtocolStacks"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):
        - `attachedMQTTClient`: 关联的 MQTT 客户端名称，字符串
        - `attachedSerialPort`: 关联的串口名称，字符串
        - `attachedTCPClient`: 关联的 TCP 客户端名称，字符串
        - `mqttSubscriptionConfigs`: MQTT 订阅配置列表，数组
            - `properties`: 订阅属性，键值对对象
            - `qos`: 订阅的服务质量等级
            - `topicSuffix`: 订阅的主题后缀，字符串
            - `unsubscriptionProperties`: 取消订阅属性，键值对对象
        - `name`: 协议栈名称，字符串
        - `outputFormat`: 输出格式，枚举值之一
        - `type`: 协议栈类型，枚举值之一

             ```json
                [
                  {
                    "result": [
                      {
                        "attachedMQTTClient": "ReservedClient",
                        "attachedSerialPort": "",
                        "attachedTCPClient": "",
                        "mqttSubscriptionConfigs": [
                          {
                            "properties": {},
                            "qos": 2,
                            "topicSuffix": "protocol_stack/Control/ReservedControlProtocolStack/+/request",
                            "unsubscriptionProperties": {}
                          }
                        ],
                        "name": "ReservedControlProtocolStack",
                        "outputFormat": "JSON",
                        "type": "Control"
                      }
                    ]
                  }
              ]
            ```

- `availableProtocolType`: 获取支持的协议栈类型列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableProtocolType"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "Control",
                "JB3208",
                "DTU"
              ]
            }
          ]
         ```

- `availableDataFormat`: 获取支持的数据输出格式列表
    - 请求方法和参数示例(不含通用字段):

         ```json
          [
            {
              "method": "availableDataFormat"
            },
            {
              "params": ""
            }
          ]
         ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": [
                "JSON",
                "Raw",
                "GB26875"
              ]
            }
          ]
         ```

- `registerProtocolStack`: 注册新的协议栈
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含协议栈名称、类型、输出格式和关联客户端的对象
        - `name`: 协议栈名称，字符串
        - `type`: 协议栈类型，枚举值之一

             ```json
              [
                {
                  "method": "registerProtocolStack"
                },
                {
                  "params": {
                    "name": "ExampleProtocolStack",
                    "type": "Control"
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
        [
          {
            "result": {
              "attachedMQTTClient": "",
              "attachedSerialPort": "",
              "attachedTCPClient": "",
              "mqttSubscriptionConfigs": [
                {
                  "properties": {},
                  "qos": 2,
                  "topicSuffix": "protocol_stack/Control/ExampleProtocolStack/+/request",
                  "unsubscriptionProperties": {}
                }
              ],
              "name": "ExampleProtocolStack",
              "outputFormat": "JSON",
              "type": "Control"
            }
          }
        ]
         ```

- `unregisterProtocolStack`: 注销指定的协议栈
    - 请求方法和参数示例(不含通用字段):
        - `params`: 协议栈名称，字符串

             ```json
              [
                {
                  "method": "unregisterProtocolStack"
                },
                {
                  "params": "ExampleProtocolStack"
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
          [
            {
              "result": ""
            }
          ]
         ```

- `setProtocolStack`: 设置指定协议栈的参数
    - 请求方法和参数示例(不含通用字段):
        - `params`: 包含协议栈名称、类型、输出格式和关联客户端的对象
        - `name`: 协议栈名称，字符串
        - `outputFormat`: 输出格式，枚举值之一
        - `attachedSerialPort`: 关联的串口名称，字符串
        - `attachedTCPClient`: 关联的 TCP 客户端名称，字符串
        - `attachedMQTTClient`: 关联的 MQTT 客户端名称，字符串

             ```json
              [
                {
                  "method": "setProtocolStack"
                },
                {
                  "params": {
                    "name": "ExampleProtocolStack",
                    "outputFormat": "JSON",
                    "attachedSerialPort": "ttyUSB0",
                    "attachedTCPClient": "example-tcp-client",
                    "attachedMQTTClient": "example-mqtt-client"
                  }
                }
              ]
             ```

    - 成功响应结果示例(不含通用字段):

         ```json
        [
          {
            "result": {
              "attachedMQTTClient": "",
              "attachedSerialPort": "",
              "attachedTCPClient": "",
              "mqttSubscriptionConfigs": [
                {
                  "properties": {},
                  "qos": 2,
                  "topicSuffix": "protocol_stack/Control/ExampleProtocolStack/+/request",
                  "unsubscriptionProperties": {}
                }
              ],
              "name": "ExampleProtocolStack",
              "outputFormat": "JSON",
              "type": "Control"
            }
          }
        ]
         ```

### 保留协议栈

#### ReservedControlProtocolStack

- **协议栈类型**: `Control`
- **协议栈名称**: `ReservedControlProtocolStack`
- **关联的 MQTT 客户端**: `ReservedClient`
- **输出格式**: `JSON`
- **说明**: 该协议栈用于处理控制指令，通过 MQTT 客户端 `ReservedClient` 进行通信
