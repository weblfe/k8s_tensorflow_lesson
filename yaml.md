# YAML 配置文件语法 简介

YAML（/ˈjæməl/，尾音类似camel骆驼）是一个可读性高，用来表达数据序列的格式。YAML参考了其他多种语言，包括：C语言、Python、Perl，并从XML、电子邮件的数据格式（RFC 2822）中获得灵感。上述介绍引用自维基百科，通俗的讲，YAML是专门用来写配置文件的语言，非常简洁和强大，远比 JSON/XML格式方便。YAML允许在层次结构中存储结构化数据。YAML 旨在以人为和机器可读，并且开销最小。YAML 规范可以在 yaml.org 找到，官方还提供了一个便捷的YAML小抄。

### YAML 详细 语法 规则 参考 可查看 [YAML官方说明](https://yaml.org/spec/1.2.2/)

# YAML的基本语法规则

## 缩进

使用空格作为缩进标记，不可以使用tab
使用缩进表征层级关系，缩进空格数不做限制，但要求同一层级左对齐

## 大小写
YAML对大小写敏感

## 注释
使用#表明注释，从*字符一直到行尾，都会被解析器忽略

## 多文档支持

YAML支持使用三个破折号分割文档 `---` 

`---` 上下两部分会作为两个独立的文档同等对待
```yaml
apiVersion: v1
kind: Service
metadata:
  name: tensor-serv-svc
  namespace: tensor
spec:
  ports:
    - port: 8080
  selector:
    app: tensor-serv
---
apiVersion: autoscaling/v2beta1
kind: HorizontalPodAutoscaler
metadata:
  name: tensor-serv-hpa-c
  namespace: tensor
  labels:
    app: tensor-serv-hpa-c
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: tensor-serv
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      targetAverageUtilization: 80
```

## 区块字符

再次强调，字符串不需要包在引号之内。有两种方法书写多行文字（multi-line strings），一种可以保存新行（使用|字符），另一种可以折叠新行（使用>字符）
```yaml
# 保存新行(Newlines preserved) 和保存新行不同的是，换行字符会被转换成空白字符。而引领空白字符则会被自动消去。
data: |                                     # 译者注：這是一首著名的五行民谣
   There once was a man from Darjeeling     # 这里曾有一个人來自大吉领
   Who got on a bus bound for Ealing        # 他搭上一班往伊灵的公车
       It said on the door                  # 门上这么说的
       "Please don't spit on the floor"     # "请勿在地上吐痰"
   So he carefully spat on the ceiling      # 所以他小心翼翼的吐在天花板上
折叠新行(Newlines folded)
--- 
data: >
   Wrapped text         # 摺疊的文字
   will be folded       # 將會被收
   into a single        # 進單一一個
   paragraph            # 段落
   
   Blank lines denote   # 空白的行代表
   paragraph breaks     # 段落之间的间隔
```

## 数据结构

常量

常量值最基本的数字、字符串、布尔值等等
```yaml
number-value: 42
floating-point-value: 3.141592
boolean-value: true
# 对于字符串，在YAML中允许不添加引号。
string-value: 'Bonjour'
```

字典
使用：表征一个键值对 my_key: my_value
冒号后面必须间隔至少一个空格

```yaml
my_key: my_value
--- 
#或者表达为：
my_key:
  my_value
```

在python中，会将上述键值对解析为一个字典
```yaml
{'my_key': 'my_value'}
```

上述键值对可以嵌套，从而实现更加复杂的数据结构：
```yaml
first_level_dict_key:
  second_level_dict_key: value_in_second_level_dict
```
在python中，会将其解析为嵌套的字典结构：
```json
{
    "first_level_dict_key": {
        "second_level_dict_key": "value_in_second_level_dict"
    }
}
```

字典嵌套列表
破折号可用于描述列表结构
破折号后必须有空格做间隔
列表和键值对嵌套使用:
```yaml
my_dictionary:
  - list_value_one
  - list_value_two
  - list_value_three
```

在python中，会将其解析为字典嵌套列表的复合数据结构：
```json
{"my_dictionary": ["list_value_one", "list_value_two", "list_value_three"]}
```

## 高级用法

数据合并和参考
为了维持文件的简洁，并避免数据输入的错误，YAML提供了`结点参考（*）` 和`散列合并（<<）`参考到其他结点标签的`锚点标记`（&）。

参考会将树状结构加入锚点标记的内容，并可以在所有数据结构中运作（可以参考上面"ship-to"的示例）合并只有散列表可以使用，可以将键值自锚点标记复制到指定的散列表中。当数据被instantiate合并和参考会被剖析器自动展开。

```yaml
defaults: &defaults
  adapter:  postgres
  host:     localhost

development:
  database: myapp_development
  <<: *defaults

test:
  database: myapp_test
  <<: *defaults
```

等同于下面的代码 ： 
```yaml
defaults:
  adapter:  postgres
  host:     localhost

development:
  database: myapp_development
  adapter:  postgres
  host:     localhost

test:
  database: myapp_test
  adapter:  postgres
  host:     localhost
```

> 语法小结

```yaml
%YAML 1.1   # Reference card
---
Collection indicators:
    '? ' : Key indicator.
    ': ' : Value indicator.
    '- ' : Nested series entry indicator.
    ', ' : Separate in-line branch entries.
    '[]' : Surround in-line series branch.
    '{}' : Surround in-line keyed branch.
Scalar indicators:
    '''' : Surround in-line unescaped scalar ('' escaped ').
    '"'  : Surround in-line escaped scalar (see escape codes below).
    '|'  : Block scalar indicator.
    '>'  : Folded scalar indicator.
    '-'  : Strip chomp modifier ('|-' or '>-').
    '+'  : Keep chomp modifier ('|+' or '>+').
    1-9  : Explicit indentation modifier ('|1' or '>2').
           # Modifiers can be combined ('|2-', '>+1').
Alias indicators:
    '&'  : Anchor property.
    '*'  : Alias indicator.
Tag property: # Usually unspecified.
    none    : Unspecified tag (automatically resolved by application).
    '!'     : Non-specific tag (by default, "!!map"/"!!seq"/"!!str").
    '!foo'  : Primary (by convention, means a local "!foo" tag).
    '!!foo' : Secondary (by convention, means "tag:yaml.org,2002:foo").
    '!h!foo': Requires "%TAG !h! <prefix>" (and then means "<prefix>foo").
    '!<foo>': Verbatim tag (always means "foo").
Document indicators:
    '%'  : Directive indicator.
    '---': Document header.
    '...': Document terminator.
Misc indicators:
    ' #' : Throwaway comment indicator.
    '`@' : Both reserved for future use.
Special keys:
    '='  : Default "value" mapping key.
    '<<' : Merge keys from another mapping.
Core types: # Default automatic tags.
    '!!map' : { Hash table, dictionary, mapping }
    '!!seq' : { List, array, tuple, vector, sequence }
    '!!str' : Unicode string
More types:
    '!!set' : { cherries, plums, apples }
    '!!omap': [ one: 1, two: 2 ]
Language Independent Scalar types:
    { ~, null }              : Null (no value).
    [ 1234, 0x4D2, 02333 ]   : [ Decimal int, Hexadecimal int, Octal int ]
    [ 1_230.15, 12.3015e+02 ]: [ Fixed float, Exponential float ]
    [ .inf, -.Inf, .NAN ]    : [ Infinity (float), Negative, Not a number ]
    { Y, true, Yes, ON  }    : Boolean true
    { n, FALSE, No, off }    : Boolean false
    ? !!binary >
        R0lG...BADS=
    : >-
        Base 64 binary value.
Escape codes:
 Numeric   : { "\x12": 8-bit, "\u1234": 16-bit, "\U00102030": 32-bit }
 Protective: { "\\": '\', "\"": '"', "\ ": ' ', "\<TAB>": TAB }
 C         : { "\0": NUL, "\a": BEL, "\b": BS, "\f": FF, "\n": LF, "\r": CR,
               "\t": TAB, "\v": VTAB }
 Additional: { "\e": ESC, "\_": NBSP, "\N": NEL, "\L": LS, "\P": PS }
...
```

### 更详细教程 [阮一峰.YAML 语言教程](https://www.ruanyifeng.com/blog/2016/07/yaml.html)

> 文章内容大部分 来自 作者：北春南秋
链接：https://www.jianshu.com/p/20c3cde0f189
来源：简书
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
