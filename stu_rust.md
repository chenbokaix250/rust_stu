# RUST语言

## RUST概述
RUST是一种通用的编程语言,它更善于以下场景:
* 需要运行时的速度
* 需要内存安全
* 更好的利用多处理器


RUST擅长的领域
* Web Service
* WebAssembly
* 命令行工具
* 网络编程
* 嵌入式设备
* 系统编程

RUST优势:性能/安全性/并发
缺点:难学

## 构造器 cargo 
利用 `cargo new xxx` 创建项目
利用`cargo build` 编译项目
利用`cargo run` 运行项目
`cargo check` 检查代码,确保能通过编译,但是不产生任何可执行文件 
`cargo check`比`cargo build`快很多 ,编写代码的时候可以连续反复的使用`cargo check`检查代码,提高效率.
`cargo build --release`用于发布构建


## 通用的编程概念

变量与可变性
* 声明变量使用let关键字
* 默认情况下,变量是不可变的(Immutable)


### 常量

常量(constant)
常量通常在绑定值以后是不可变的,与变量的区别是:
* 不可使用mut,常量永远不可变
* 声明常量使用const关键字,它的类型必须被标注
* 常量可以在任何作用域内进行声明,包括全局作用域
* 常量只可以绑定到常量表达式,无法绑定到函数的调用结果或只能在运行时才能计算出的值

* 程序运行期间,常量在其声明的作用域内一直有效
* **命名规范**:Rust的常量使用全大写字母 用下划线连接.

shadowing机制
可以使用相同的名字声明新的变量,新的变量就会shadow(隐藏)之前声明的同名变量.
shadow和把变量标记为mut是不一样的:
* 如果不适用let关键字,那么重新给非mut的变量赋值会导致编译时错误
* 使用let声明的同名新变量,也是不可变的
* 使用let声明的同名新变量,它的类型可以与之前不同

### 数据类型

标量类型和复合类型

Rust是静态编译语言,在编译时需要知道所有变量的类型
* 基于使用的值,编译器通常能够推断出它的具体类型
* 如果可能的类型比较多,就必须添加类型的标注,否则编译会报错

#### 标量类型

一个标量类型代表一个单个的值
Rust有四个主要的标量类型:
1. 整数类型
2. 浮点类型
3. 布尔类型
4. 字符类型

整数类型: 
无符号整数类型以U开头 
有符号整数类型以i开头

整数字面值
|Number literals|Example|
|----|----|
|Decimal|98_220|
|Hex|0xff|
|Octal|0off|
|Binary|0b1111_0000|
|Byte(u8 only)|b'A'|

字符类型:
Rust语言中char类型被用来描述语言中最基础的单字符
* 字符类型的字面值使用单引号
* 占用4字节大小
* 是Unicode标量值,可以表示更多内容

#### 复合类型

复合类型可以将多个值放在一个类型里
Rust提供了两种基础的复合类型:元祖(Tuple),数组

##### Tuple
Tuple可以将多个类型的多个值放在一个类型里
Tuple的长度是固定的:一旦声明就无法改变

获取Tuple的元素值:
通过使用模式匹配来解构(destructure)一个Tuple来获取元素的值.

访问Tuple的元素
在tuple变量使用点标记法,后接元素的索引号

##### 数组
数组也可以将多个值放在一个类型里
数组中每个元素的类型必须相同
数组的长度也是固定的
用途:
如果想让数据存放在stack(栈)上而不是heap(堆)上,或者想保证有固定数量的元素,这时使用数组更有好处.  

vector由标准库提供 比数组灵活


### 函数

声明函数使用fn关键字
针对函数和变量名,Rust使用`snake case`命名规范,所有字母都是小写,单词之间使用下划线分开
函数的参数
函数签名里,必须声明每个参数的类型

函数的返回值 
在->符号后边声明函数返回值的类型,但是不可以为返回值命名
在Rust里面,返回值就是函数体里面最后一个表达式的值
若想提前返回,需要使用return关键字,并指定一个值

注释与c语言一致

if的条件语句必须是bool类型

使用else if处理多重条件

使用多个else,最好使用match来重构代码

### 循环
Rust提供了3种循环:
* loop
* while
* for

#### loop循环

loop关键字高速Rust反复执行一块diamante,直到break关键字告诉程序停止.
#### while条件循环
while每次执行循环体之前都判断一次条件.
#### for循环遍历集合
利用for来遍历数组 速度和正确性

Range(标准库)
指定一个开始数字和一个结束数字,range可以生成它们之间的数字(不含结束)
rev方法用来反转Range

---

## 所有权

所有权是Rust最独特的特性,让Rust无需GC就可以保证内存安全

Rust的核心特性就是所有权
所有程序在运行时都必须管理它们使用计算机内存的方式
* 有些语言有垃圾收集机制,在程序运行时,它们会不断地寻找不再使用的内存
* 在其他语言中,程序员必须显式地分配和释放内存

Rust采用了第三种方式:
内存是通过一个所有权系统来管理的,其中包含一组编译器在编译时检查的规则
当程序运行时,所有权特性不会减慢程序的运行速度.

**Stack(栈内存) Heap(堆内存)**
在Rust中,一个值是在stack上还是在heap上对语言的行为和你为什么要做某些决定是有更大的影响的
在代码运行时,Stack和heap都是你可用的内存,但结构不同

stack存储数据特性
stack按值的接收顺序来存储,按相反的顺序将它们移除(后进先出 LIFO)
* 添加数据叫做压入栈
* 移除数据叫做弹出栈

所有存储在stack上的数据必须拥有已知的固定的大小
编译时大小未知的数据或运行时大小可能发生变化的数据必须存放在heap上

Heap内存组织性差一些:
当把数据放入heap时,会请求一定数量的空间
操作系统在heap里找到一块足够大的空间,把它标记为在用,并返回一个指针,也就是这个空间的地址.
这个过程叫做在heap上进行分配,有时仅称为分配

heap中的数据要比访问stack中的数据慢

#### 所有权存在的原因

所有权解决的问题:
* 跟踪代码的哪些部分正在使用heap的哪些数据
* 最小化heap上的重复数据量
* 清理heap上未使用的数据以避免空间不足
管理heap数据是所有权存在的原因.

### 所有权规则
每个值都有一个变量,这个变量是该值的所有者.
每个值同时只能有一个所有者
当所有者超出作用域时,该值将被删除

作用域是一个项目的有效范围


内存和分配
字符串字面值,在编译时内容写死,速度快但其不可变.
String类型为了支持可变性,需要在heap上分配内存来保存编译时未知的文本内容:

* 操作系统必须在运行时来请求内存(通过调用String::from来实现)
* 当用完String后,需要使用某种方式将内存返回给操作系统(GC所做的工作)
Rust采用了不同的方式:对于某个值来说,当拥有它的变量走出作用范围时,内存会立即自动的交还给系统;

drop函数

变量和数据交互的方式:移动(move)

---
###  引用与借用

参数的类型是`&String`而不是String
&符号表示引用:允许你引用某些值而不取得其所有权.

**借用**:把引用作为函数参数这个行为叫做借用

和变量一样,引用默认是不可变的.

可变引用:
`let s_ref = &mut s`
可变引用有一个重要的限制:在特定作用域内,对某一块数据,只能有一个可变的引用.
可以通过创建新的作用域,来允许非同时的创建多个可变引用.
不可以同时拥有一个可变引用和一个不变的引用.

**引用的规则**
在任何给定的时刻,只能满足下列条件之一:
* 一个可变的引用
* 任意数量不可变的引用

---

### 切片
切片在Rust中是一种不持有所有权的数据类型
 
**字符串切片**
字符串切片是指向字符串中一部分内容的引用
形式: [开始索引..结束索引]
- 开始索引是切片起始位置的索引值
- 结束索引是切片终止位置的下一个索引值

---

## struct

### 定义并实例化struct

struct 结构体
定义struct
使用struct关键字,并为整个struct命名
在花括号内,为所有字段(Field)定义名称和类型

实例化struct,可以不用顺序初始化

取得struct里面的某个值,使用点标记法.
**一旦struct的实例是可变的,那么实例中所有的字段都是可变的**

字段初始化简写
当字段名与字段值对应变量名相同时,就可以使用简写的方式
```rust
fn build_user(email: String,username: String) -> User{
    User {
        email, //email:email 变量名与形参同名可以简写
        username,//username:username
        active:True,
        sign_in_count:0,
    }
}
```

### struct更新语法
当想基于某个struct实例来差UN构建一个IE新实例的时候,可以使用### struct更新语法
```rust
//基于user1构建新的user2
let user2 = User{
    emain:String::from("3035@qq.com"),
    username:String::from("anthoni"),
    ..user1
}
```

### Tuple struct 
可以定义类似tuple的struct
Tuple struct整体有个名字,里面的元素没有名字.
定义tuple struct,使用struct关键字,后边是名字,以及里面的元素类型
`struct Color(i32,i32,i32)`

### Unit-Like Struct
可以定义没有任何字段的struct

### struct数据的所有权

声明struct使用String而不是&str:
该struct实例拥有其所有的数据
只要struct实例是有效的,那么里面的字段数据也是有效的

struct里也可以存放引用,但需要使用生命周期

### struct的打印
std::fmt::Display无法直接打印
需要借助于`std::fmt::Debug`
利用`#[derive(Debug)]`注解
`{:?} && {:#:}` debug打印时使用的格式 

### struct的方法

方法和函数类似:fn关键字/名称/参数/返回值
定义方法:
在impl块里定义方法.
方法的第一个参数可以是&self,也可以获得其所有权或可变借用.和其他参数一样
更良好的代码组织.
方法调用的运算符
在调用方法时,Rust根据情况自动添加&,&mut或*,以便object可以匹配方法的签名

### 关联函数

可以在impl块里定义不把self作为第一个参数的函数,他们叫关联函数
关联函数通常用于构造器
`::`符号
- 关联函数
- 模块创建的命名空间

### 多个impl块

每个struct允许拥有多个impl块

---

## 枚举

### enum

枚举允许列举所有可能的值来定义一个类型
`enum 标识符{1,2}`

允许将数据附加到枚举的变体中
优点:
1. 不需要额外使用struct
2. 每个变体可以拥有不同的类型以及关联的数据量

### Option 枚举
* Rust没有Null
* Rust中类似Null的枚举 Option<T>
Option<i32> 与i32 不是一个类型
若想使用Option<T>中的T,必须将它转换为T


### match 控制流运算符

match匹配时,必须穷举所有可能

_通配符:替代其余没列出的值

### if let 
处理只关心一种匹配而忽略其它匹配的情况
```rust
match v{
    Some(3) => println!("three"),
    _=>(),
}
//if let
if let Some(3) = v{println!("three:);}
```
也就放弃了穷举的可能
if let看作是match的语法糖

---

## Package crate module

Rust的代码组织:
代码组织主要包括:
哪些细节可以暴露,哪些细节是私有的
作用域内哪些名称有效

模块系统:
**package**:包,Cargo的特性,让你构建/测试/共享crate
**Crate**:单元包,一个模块树,可产生一个library或可执行文件
**Module**:use 控制代码的组织/作用域/私有路径
**Path**: 路径,为struct,function或module等命名的方式

Crate的类型:
binary
library
CrateRoot:
是源代码文件
Rust编译器是从这里开始 组成你得Crate的根Module

一个Package:
* 包含1个Cargo.toml,它描述如何构建这些Crates
* 只能包含0-1个library crate
* 可以包含任意数量的binary crate
* 但必须至少包含一个crate


cargo的惯例

src/main.rs
-binary crate的crate root
-crate名与package名相同

src/lib.rs
-package包含一个library crate
-library crate的crate root
-crate名与package名相同

cargo把crate root文件交给rustc来构建library或binary

一个package可以同时包含main.rs和lib.rs
一个binary crate 一个library crate

crate的作用
将相关功能组合到一个作用域内,便于在项目间共享(防止冲突)

**定义module来控制作用域和私有性**

Module,在一个crate内,将代码进行分组
控制项目的私有性,public,pricate

建立module:
mod关键字
可嵌套
可包含其他项,(struct,enum,常量,trait,函数等)的定义
模块不仅可以组织代码,还可以定义私有边界
如果想把函数或者struct等设为私有,可以将它放到某个模块中.
**rust中所有的条目(函数/方法/struct/enum/模块/常量)默认都是私有的**
父级模块无法访问子模块中的私有条目
子模块里可以使用所有祖先模块中的条目

super关键字
super用来访问父级模块路径中的内容,类似文件系统中的 `..`

pub struct 可以用于声明公共结构体(成员仍然私有)
pub enum 可以用于声明公共枚举(变体为公共)
**struct与enum 规则不同**

### use 关键字
可以使用use关键字将路径导入到作用域内
### as 关键字
as关键字可以为引入的路径指定本地的别名

使用`pub use`重新导出名称
使用use将路径导入到作用域内后,该名称在此作用域内是私有的.
pub use:重导出
* 将条目引入作用域
* 该条目可以被外部代码引入到它们的作用域

### 使用外部包(package)
1. Cargo.toml添加依赖的包(package)
2. use将特定条目引入作用域

标准库(std)也被当做外部包,但是不需要在dependence下添加
通配符 * 可以把路径中所有的公共条目都引入到作用域

将模块内容移动到其他文件

模块定义时,如果模块名后备是;,而不是代码块
* Rust会从与模块同名的文件中加载内容
* 模块树的结构不会变化

随着模块逐渐变大,该技术让你可以把模块的内容移动到其他文件中.

---

## 8.常用的集合

### vector
使用Vector存储多个值
Vec<T>,叫做vector
* 由标准库提供
* 可存储多个值
* 只能存储相同类型的数据
* 值在内存中连续存放

利用push进行添加元素
当离开作用域后,vect会被清零

读取vector的值
1.利用引用
2.get方法

所有权和借用规则
* 不能在同一作用域内同时拥有可变和不可变引用

利用enum配合vector 可以实现对enum的可变操作

### string类型

Rust倾向于暴露可能的错误
字符串数据结构复杂
utf-8

字符串是byte的集合,能将byte解析为文本

Rust核心语言层面,只有一个字符串类型:字符串切片str(或&str)

String类型来自标准库,而不是核心语言

创建一个新的字符串String
* 很多Vector<T>的操作都可用于String
* String::new()函数
* 使用初始值来创建String;
    1.利用to_string方法
    2.使用String::from函数
* 更新String
    1. push_str()方法:把字符串切片,进行附加
    2. push()方法:把单个字符附加
    3. +运算符:对字符串进行拼接(解引用强制转换)
* format! 进行拼接

对String按索引的形式进行访问
rust语法不支持
string是对Vector<u8>类型的包装.

RUst有三种看待字符串的方式:
- 字节
- 标量值
- 字形簇

切割String
利用`&str[0..4]`切片获取

### HasgMap<K,V>
键值对的形式存储数据,一个键对应一个值
Hash函数:决定如何在内存中存放K和V

HashMap:
* HashMap用的较少,不在Prelude中
* 标准库对其支持较少,没有内置的宏来创建HashMap
* 数据存储在heap上
* 同构性:
    所有K必须是同一种类型

利用collect方法创建HashM,利用拉链函数组装

访问HashMap中的值
get方法,参数K,返回Option<&V>

更新HashMap时,利用entry方法,可以确认是否存在K,如果存在,不插入更新,不存在则插入更新.
Entry的or_intert()方法
