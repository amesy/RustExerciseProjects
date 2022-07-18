# 新建workspace
```bash
% mkdir exerciseProjects
% cd exerciseProjects 
% vim Cargo.toml
[workspace]
member = {
    "snake",
}
```

# 新建项目

```bash
// 创建一个项目
cargo new snake 
```

# 添加依赖 

```toml
// Cargo.toml
[dependencies]
#随机数，用于处理apple的随机数，星号表示使用该依赖包的最新版本
rand = "*"
#活塞窗口，这将允许我们使用UI渲染元素以及处理一些游戏逻辑
piston_window = "*"

> 编辑完Cargo.toml文件后，运行命令`cargo update`会根据Cargo.toml的信息生成确切依赖信息和Cargo.lock文件，并保存在该文件中。
这时，就可以从Cargo.lock文件中获取依赖包的确切最新版本信息，填写到Cargo.toml文件。
使用静态版本很重要的原因是为了防止依赖库实际发生变化，如果语法发生变化，项目就运行不起来了。

// Cargo.toml
[dependencies]
rand = "0.8.4"
piston_window = "0.121.0"
```

# 构建本地包及其所有依赖项 

```bash
// 运行命令
cargo build
```
