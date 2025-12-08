<p align="center">
  <img src="https://img.shields.io/badge/Rust-🦀-orange?style=for-the-badge" alt="Rust"/>
  <img src="https://img.shields.io/badge/Gemini-AI-blue?style=for-the-badge" alt="Gemini AI"/>
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="MIT License"/>
</p>

<h1 align="center">🚀 The Prompt of Prompt</h1>

<p align="center">
  <em>用 AI 生成更好提示词的元提示词工具</em>
</p>

<p align="center">
  <strong>使用 Google Gemini AI 将你的普通提示词转化为专业的高性能提示词</strong>
</p>

<p align="center">
  <a href="./README.md">🇺🇸 English</a> •
  <a href="#快速开始">快速开始</a> •
  <a href="#功能特性">功能特性</a> •
  <a href="#工作原理">工作原理</a>
</p>

---

## ⭐ 为什么要给这个项目 Star？

> **写好提示词真的很难。** 即使是经验丰富的开发者也很难写出能持续获得最佳 AI 响应的提示词。

这个工具通过使用 **Google 最先进的 AI（Gemini 3 Pro）** 自动按照专业提示词工程最佳实践重写你的提示词，解决了这个问题。

**如果你觉得有用，请给我们一个 ⭐ Star！** 这将帮助更多开发者发现这个工具。

---

## 🎯 问题是什么？

你写了一个这样的提示词：
```
帮我写一个处理 csv 文件的 python 脚本
```

然后奇怪为什么 AI 给你的结果总是很泛泛、不完整或不一致。

## ✨ 解决方案

**The Prompt of Prompt** 将其转化为专业提示词：

```markdown
<persona>
你是一位拥有 10 年以上数据处理经验的高级 Python 开发者，
专精于 pandas、csv 处理和自动化脚本。
</persona>

<context>
我需要一个用于数据分析的 CSV 文件处理 Python 脚本。
</context>

<task>
创建一个 Python 脚本，要求：
1. 从指定目录读取 CSV 文件
2. 处理常见问题（编码、缺失值、格式错误的行）
3. 提供汇总统计信息
4. 导出清洗后的数据
</task>

<constraints>
- 仅使用标准库 + pandas
- 包含适当的错误处理
- 添加调试日志
- 遵循 PEP 8 代码风格
</constraints>

<output_format>
提供完整的 Python 脚本，包含：
- 解释每个部分的清晰注释
- 文档字符串中的使用示例
- 所有函数的类型提示
</output_format>
```

**结果：** AI 响应更准确、更完整、更一致。

---

## 🔥 功能特性

| 功能 | 描述 |
|------|------|
| **🎯 多版本生成** | 使用 Batch API 生成 N 个优化版本（默认 3 个，可配置） |
| **🏆 大师级提示词** | 自动合并最佳元素生成一个优化的最终提示词 |
| **💰 节省 50%** | 使用 Gemini Batch API 节省成本 |
| **🧠 Gemini 3 Pro** | 由 Google 最智能的 AI 模型驱动 |
| **⚡ 极速** | 批处理在几秒钟内完成 |
| **🌍 双语支持** | 支持中文和英文界面 |

---

## 📦 安装

### 前提条件

- [Rust](https://rustup.rs/) (1.70+)
- Google Gemini API 密钥（[免费获取](https://aistudio.google.com/apikey)）

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/anthropics/the-prompt-of-prompt.git
cd the-prompt-of-prompt

# 构建项目
cargo build --release

# 二进制文件位于 ./target/release/prompt-improver
```

### 设置 API 密钥

```bash
export GEMINI_API_KEY="你的-api-密钥"
```

---

## 🚀 快速开始

```bash
# 运行提示词优化器
cargo run --package prompt-improver

# 或者使用构建好的二进制文件
./target/release/prompt-improver
```

### 示例会话

```
╔══════════════════════════════════════════════════════════════╗
║              🚀 Gemini 提示词优化器                           ║
║            将你的提示词转化为专业提示词                         ║
╚══════════════════════════════════════════════════════════════╝

📝 请输入你的草稿提示词（按两次回车提交）：
───────────────────────────────────────────────────────────────
写一个解析 json 的 rust 函数

⏳ 正在使用 Gemini 3 Pro 优化你的提示词（Batch API - 3个版本）...
   💡 使用 Batch API：比实时请求便宜 50%！

📤 正在创建包含 3 个请求的批处理任务...
✅ 批处理任务已创建: batchJobs/abc123
⏳ 等待批处理任务完成...
📥 批处理任务已完成！

💾 版本 1 已保存到: improved_prompt_1.md
💾 版本 2 已保存到: improved_prompt_2.md
💾 版本 3 已保存到: improved_prompt_3.md

🔀 正在将 3 个版本合并为大师级提示词...
✅ 大师级提示词已保存到: master_prompt.md

═══════════════════════════════════════════════════════════════
🏆 最终输出:
   • improved_prompt_1.md  (版本 1)
   • improved_prompt_2.md  (版本 2)
   • improved_prompt_3.md  (版本 3)
   • master_prompt.md      (✨ 合并优化版)
═══════════════════════════════════════════════════════════════

📊 总 Token 使用量（3 个批处理 + 1 个合并请求）:
   输入:    1,234 tokens
   输出:    5,678 tokens
   总计:    6,912 tokens

💰 预估成本（Batch + 实时 API）:
   总计:    $0.012345
```

---

## 🛠️ 工作原理

```
┌─────────────────┐
│  你的草稿       │
│  提示词         │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────────────────┐
│              Gemini 3 Pro (Batch API)               │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐             │
│  │ 版本 1  │  │ 版本 2  │  │ 版本 3  │             │
│  └─────────┘  └─────────┘  └─────────┘             │
└─────────────────────────────────────────────────────┘
         │           │           │
         └─────────┬─────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│              Gemini 3 Pro (合并)                     │
│           综合三个版本的最佳元素                      │
└─────────────────────────────────────────────────────┘
                   │
                   ▼
         ┌─────────────────┐
         │  大师级提示词    │
         │  (已优化!)       │
         └─────────────────┘
```

### 应用的提示词工程原则

1. **📋 清晰结构** - 使用 XML 标签或 Markdown 分隔各部分
2. **🎭 角色定义** - 分配特定的专家角色
3. **📝 少样本示例** - 在有帮助时添加输入/输出示例
4. **🚫 负面约束** - 明确说明不要做什么
5. **📄 输出格式** - 定义确切的响应结构
6. **🧠 思维链** - 添加分步推理指令

---

## 🌍 语言支持

CLI 支持中文和英文界面：

```bash
# 英文（默认）
export LANG=en_US.UTF-8
cargo run --package prompt-improver

# 中文
export LANG=zh_CN.UTF-8
cargo run --package prompt-improver
```

---

## 📁 项目结构

```
the-prompt-of-prompt/
├── crates/
│   ├── gemini-client/      # 🔧 Gemini API SDK（功能完整）
│   │   ├── src/
│   │   │   ├── client.rs   # API 客户端实现
│   │   │   ├── types.rs    # 请求/响应类型
│   │   │   └── models.rs   # 模型常量
│   │   └── Cargo.toml
│   │
│   └── prompt-improver/    # 🚀 CLI 工具（本工具！）
│       ├── src/
│       │   └── main.rs     # 主应用程序
│       └── Cargo.toml
│
├── docs/                   # 📚 文档
│   ├── core capabilities/  # Gemini API 功能
│   └── prompt engineering/ # 提示词工程指南
│
├── README.md               # 📖 英文文档
├── README_CN.md            # 🇨🇳 本文件
└── Cargo.toml              # 工作空间配置
```

---

## 🔧 配置

### 环境变量

| 变量 | 描述 | 必需 |
|------|------|------|
| `GEMINI_API_KEY` | 你的 Google Gemini API 密钥 | ✅ 是 |
| `PROMPT_VERSION_COUNT` | 生成提示词版本数量（默认：3） | 否 |
| `HTTPS_PROXY` | 代理服务器（如需要） | 否 |
| `LANG` | 语言（`en_US.UTF-8` 或 `zh_CN.UTF-8`） | 否 |

---

## 📊 Gemini 客户端 SDK

本项目包含一个功能完整的 Gemini API Rust SDK：

```rust
use gemini_client::{GeminiClient, GenerateContentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GeminiClient::from_env()?;
    
    let response = client.generate_content(
        GenerateContentRequest::with_text("你好, Gemini!")
    ).await?;
    
    println!("{}", response.text().unwrap_or("无响应"));
    Ok(())
}
```

### SDK 功能

- ✅ 文本生成（支持流式）
- ✅ 图像生成（Gemini 3, Imagen 4）
- ✅ 视频生成（Veo 2, 3, 3.1）
- ✅ 音频理解和语音合成
- ✅ 文档/PDF 处理
- ✅ 视觉和目标检测
- ✅ 嵌入向量（多种任务类型）
- ✅ 上下文缓存（隐式和显式）
- ✅ Batch API（节省 50% 成本）
- ✅ 函数调用
- ✅ 结构化输出（JSON Schema）

---

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建你的功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m '添加某个很棒的功能'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

## ❓ 常见问题

**问：如果我是提示词工程的新手，应该从哪里开始？**

答：从问自己这个问题开始：*什么是好的提示词？* 好的提示词应该是清晰、具体，并提供足够上下文让 AI 理解你的意图。尝试定义角色、任务、约束条件和期望的输出格式。你可以先输入一个简单的草稿提示词，让这个工具引导你走向更专业的版本！

**问：我应该直接使用大师级提示词吗？**

答：大多数情况下可以直接使用！大师级提示词融合了所有生成版本的最佳元素。不过，你可能需要根据具体使用场景进行一些微调，比如调整角色设定、添加特定领域的约束条件，或修改输出格式。

---

## ⭐ Star 历史

如果这个项目帮助了你，请考虑给它一个 star！⭐

你的支持帮助我们继续改进这个工具，并保持我们的动力！

---

<p align="center">
  用 ❤️ 制作，来自 Prompt of Prompt 团队
</p>

<p align="center">
  <a href="https://github.com/anthropics/the-prompt-of-prompt/issues">报告 Bug</a> •
  <a href="https://github.com/anthropics/the-prompt-of-prompt/issues">请求功能</a>
</p>
