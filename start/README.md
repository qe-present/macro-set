# start目录


# 宏
因为宏的参数可以使用 ()、[] 以及 {}
在 Rust 中宏分为两大类：
1. 声明宏( declarative macros ) macro_rules! 
2. 过程宏( procedural macros ):
- 派生宏，可以为目标结构体或枚举派生指定的代码，例如 Debug 特征 
- 属性宏(Attribute-like macro)，用于为目标添加自定义的属性 
- 函数宏(Function-like macro)，看上去就像是函数调用

# 声明宏____'模板'
```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```
# 过程宏
1. 过程宏是使用源代码作为输入参数， 基于代码进行一系列操作后，再输出一段全新的代码 
2. 注意，过程宏中的 derive 宏输出的代码并不会替换之前的代码，这一点与声明宏有很大的不同 
3. 它的定义必须要放入一个独立的包中
# syn
syn 将字符串形式的 Rust 代码解析为一个 AST 树的数据结构
# quote
AST 树的数据结构转换回 Rust 代码(token stream)

# 属性宏
derive 只能用于结构体和枚举，而属性宏可以用于其它类型项，例如函数。
```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

# 函数宏
```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

# 派生宏
```rust
#[proc_macro_derive(Hello)]
pub fn hello(input: TokenStream) -> TokenStream {
```