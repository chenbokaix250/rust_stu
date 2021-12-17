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


