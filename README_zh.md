
# 待办事项 CLI 应用程序

[Click to view the English documentation](README.md)

这是一个简单的命令行界面（CLI）应用程序，用于管理待办事项列表。该应用程序使用 Rust 编写，允许用户添加、完成和列出任务。

## 功能

- 向你的待办事项列表添加新任务
- 完成待办事项列表中的任务
- 列出待办事项列表中的所有任务

## 安装

要安装此应用程序，你需要在你的机器上安装 Rust。如果你还没有安装 Rust，你可以通过 [rustup](https://rustup.rs/) 进行安装。

克隆仓库：

\```bash
git clone https://github.com/yourusername/todolist.git
cd todolist
\```

然后，使用 `cargo` 构建应用程序：

\```bash
cargo build --release
\```

可执行文件将位于 `./target/release/` 目录中。

## 使用

### 添加任务

\```bash
cargo run -- add "买牛奶"
\```

应用程序将确认任务已经被添加：

\```bash
已添加任务 1：买牛奶
\```

### 完成任务

\```bash
cargo run -- complete 1
\```

应用程序将确认任务已经完成：

\```bash
已完成任务 1：买牛奶
\```

### 列出所有任务

\```bash
cargo run -- list
\```

应用程序将列出所有任务：

\```bash
1：买牛奶
\```

## 贡献

欢迎提交 pull 请求。对于重大变更，请先开启一个 issue，讨论你想要改变的内容。

请确保适当地更新测试。