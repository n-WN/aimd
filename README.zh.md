# aimd - AI流式Markdown渲染器

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/n-WN/aimd)


一个高性能的流式Markdown渲染器，专为AI命令行工具设计。实时将AI输出渲染为带语法高亮的Markdown格式，支持所有主流AI CLI工具。

**中文README** | [English README](README.md)

[![asciicast](https://asciinema.org/a/6a0TVOzkpjiXH07AlZLnipxJH.svg)](https://asciinema.org/a/6a0TVOzkpjiXH07AlZLnipxJH)

## ✨ 特性

> 暂时在 Windows 上不可用, 不过你可以在 WSL 中使用它

- 🚀 **流式渲染** - 实时渲染AI输出，无需等待完整响应
- 🎨 **丰富格式支持** - 支持标题、列表、代码块、粗体、斜体等所有Markdown元素
- 🔧 **通用兼容** - 支持任意AI命令行工具（Gemini、Claude、OpenAI等）
- 📦 **零依赖配置** - 透明参数传递，无需适配不同工具
- 🐛 **调试模式** - 内置调试功能，方便问题排查
- 🔄 **管道友好** - 完美支持Unix管道操作
- 🌐 **多语言支持** - 自动检测系统语言（中文/英文）

## 🎯 支持的Markdown格式

| 格式 | 语法 | 渲染效果 |
|------|------|----------|
| 标题 | `# 标题` | 🔵 彩色标题 |
| 粗体 | `**粗体**` | **黄色粗体** |
| 斜体 | `*斜体*` | *斜体文字* |
| 粗斜体 | `***粗斜体***` | ***黄色粗斜体*** |
| 行内代码 | `代码` | 🟫 橘红色代码 |
| 代码块 | \`\`\`语言<br/>代码<br/>\`\`\` | 📦 带边框代码块 |
| 列表 | `• 项目` | 🟣 紫色列表 |
| 引用 | `> 引用` | 🟡 黄色边框 |

## 🚀 快速开始

### 安装

确保您已安装 Rust 1.70+：

```bash
# 克隆仓库
git clone https://github.com/n-WN/aimd.git
cd aimd

# 编译
cargo build --release

# 安装到系统
cargo install --path .
```

### 基本用法

#### 1. 查看帮助
```bash
aimd
```

#### 2. 管道输入（推荐）
```bash
echo "解释什么是Rust编程语言" | aimd
```

#### 3. 显式命令
```bash
aimd -- gemini --model gemini-2.5-flash -p "介绍Markdown语法"
```

## 📖 使用指南

### 管道模式

管道模式是最简洁的使用方式，输入内容将自动作为提示词传递给默认的Gemini：

```bash
# 简单提问
echo "什么是机器学习？" | aimd

# 文件内容作为输入
cat question.txt | aimd

# 多行输入
cat << EOF | aimd
请用Markdown格式介绍：
1. Python的特点
2. 常用库
3. 应用场景
EOF
```

### 显式命令模式

支持任意AI CLI工具，只需在 `--` 后指定完整命令：

```bash
# Gemini
aimd -- gemini --model gemini-2.5-flash -p "介绍Rust"

# Claude
aimd -- claude --model sonnet -p "解释async/await"

# OpenAI
aimd -- openai api chat.completions.create -m gpt-4 --messages '[{"role":"user","content":"Hello"}]'

# 自定义工具
aimd -- my-ai-tool --custom-param value "prompt"
```

### 调试模式

使用 `--debug` 查看详细执行信息：

```bash
# 调试管道输入
echo "测试输入" | aimd --debug

# 调试显式命令
aimd --debug -- gemini --help
```

## 🔧 配置选项

| 参数 | 描述 | 示例 |
|------|------|------|
| `--debug` | 启用调试模式，显示执行细节 | `aimd --debug` |
| `--help` | 显示帮助信息 | `aimd --help` |
| `--` | 参数分隔符，后续参数传递给AI工具 | `-- gemini -p "hello"` |

## 📊 性能特性

- **低延迟**: 流式处理，首字节响应时间 < 10ms
- **内存高效**: 行缓冲处理，内存占用恒定
- **跨平台**: 支持 macOS、Linux、Windows
- **零配置**: 开箱即用，无需配置文件

## 🛠️ 技术架构

```
输入源 → PTY进程 → 流式解析 → Markdown渲染 → 终端输出
  ↓         ↓          ↓           ↓           ↓
管道/参数  AI工具    逐行处理    ANSI着色    实时显示
```

### 核心组件

- **PTY管理**: 使用 `pty-process` 管理伪终端
- **流式解析**: 自定义状态机解析Markdown
- **ANSI渲染**: 原生ANSI转义序列着色
- **参数透传**: 零损失参数传递机制

## 🌐 语言支持

程序会根据环境变量自动检测系统语言：

- **中文**: 当 `LANG`、`LC_ALL` 或 `LC_MESSAGES` 以 `zh` 开头时
- **英文**: 其他所有情况下的默认语言

检测的环境变量（按顺序）：
1. `LANG`
2. `LC_ALL` 
3. `LC_MESSAGES`

## 🤝 AI工具兼容性

| 工具 | 状态 | 测试版本 | 说明 |
|------|------|----------|------|
| Gemini CLI | ✅ 完全支持 | latest | Google官方CLI |
| Claude CLI | ✅ 完全支持 | latest | Anthropic官方CLI |
| OpenAI CLI | ✅ 完全支持 | latest | OpenAI官方CLI |
| Ollama | ✅ 完全支持 | v0.1.0+ | 本地AI模型 |
| 自定义工具 | ✅ 完全支持 | - | 任何输出文本的命令 |

## 🎨 渲染示例

### 输入
```markdown
# AI工具比较

## 主要特点

- **Gemini**: Google的多模态AI
- **Claude**: Anthropic的对话AI  
- **GPT**: OpenAI的生成式AI

### 代码示例

```python
import openai
client = openai.OpenAI()
response = client.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello!"}]
)
print(response.choices[0].message.content)
```

> 选择合适的AI工具取决于具体需求
```

### 渲染输出
![渲染效果示例](docs/render-example.png)

## 📝 开发说明

### 构建要求

- Rust 1.70.0+
- Cargo

### 开发环境

```bash
# 克隆并进入目录
git clone https://github.com/n-WN/aimd.git
cd aimd

# 运行测试
cargo test

# 开发模式运行
cargo run -- --debug -- echo "# 测试\n**粗体文本**"

# 发布构建
cargo build --release
```

### 项目结构

```
aimd/
├── src/
│   └── main.rs              # 主程序逻辑
├── Cargo.toml              # 依赖配置
├── README.md              # 中文文档
├── README.en.md           # 英文文档
└── docs/                  # 文档资源
```

## 🐛 问题排查

### 常见问题

1. **AI工具未找到**
   ```bash
   # 确保AI工具已安装并在PATH中
   which gemini
   ```

2. **权限问题**
   ```bash
   # 确保有执行权限
   chmod +x target/release/aimd
   ```

3. **编码问题**
   ```bash
   # 设置正确的终端编码
   export LANG=en_US.UTF-8
   ```

### 调试步骤

1. 使用 `--debug` 查看详细信息
2. 检查AI工具是否正常工作
3. 验证参数传递是否正确

## 🤝 贡献指南

欢迎贡献代码！请遵循以下流程：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 添加必要的测试用例

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [pty-process](https://crates.io/crates/pty-process) - PTY管理
- [Rust社区](https://www.rust-lang.org/community) - 优秀的生态系统
- 所有贡献者和用户的支持

## 📞 联系方式

- 项目主页: [GitHub](https://github.com/n-WN/aimd)
- 问题反馈: [Issues](https://github.com/n-WN/aimd/issues)
- 功能建议: [Discussions](https://github.com/n-WN/aimd/discussions)

---

⭐ 如果这个项目对您有帮助，请给我们一个星标！ 