# 使用示例

## 基础示例

### 简单问答
```bash
echo "什么是Rust编程语言？" | aimd
```

### 代码解释
```bash
echo "解释这段Python代码：print('Hello, World!')" | aimd
```

### 技术文档生成
```bash
cat << EOF | aimd
请用Markdown格式写一个关于REST API设计的文档，包括：
1. 基本概念
2. 设计原则  
3. 最佳实践
4. 示例代码
EOF
```

## 高级示例

### 文件内容分析
```bash
# 分析代码文件
cat main.rs | aimd

# 分析日志文件
cat error.log | aimd
```

### 多AI工具对比
```bash
# 使用Gemini
echo "解释量子计算" | aimd

# 使用Claude
aimd -- claude --model sonnet -p "解释量子计算"

# 使用OpenAI
aimd -- openai api chat.completions.create -m gpt-4 \
  --messages '[{"role":"user","content":"解释量子计算"}]'
```

### 本地AI模型
```bash
# 使用Ollama
aimd -- ollama run llama2 "What is machine learning?"

# 使用自定义脚本
aimd -- ./my-ai-script.sh "Hello AI"
```

## 调试和开发

### 调试模式
```bash
# 查看参数传递
echo "测试" | aimd --debug

# 调试AI工具调用
aimd --debug -- gemini --help
```

<!-- ### 性能测试
```bash
# 测试大文本渲染
yes "# 测试标题" | head -1000 | aimd -- cat

# 测试代码块渲染
cat large_source_file.rs | aimd -- cat
``` -->

## 实际应用场景

### 

```bash
(echo "尽可能使用中文回复; 问题: 如何使用 eza ; 附加:"; eza --help) | aimd
```

### 代码审查
```bash
# 分析代码质量
cat suspicious_code.py | aimd

# 生成代码注释
echo "为这段代码添加详细注释：$(cat main.rs)" | aimd
```

### 文档生成
```bash
# API文档生成
echo "根据这个函数生成API文档：$(cat api.py)" | aimd

# README生成
echo "为这个项目生成README：$(ls -la)" | aimd
```

### 学习辅助
```bash
# 概念解释
echo "用简单的话解释区块链技术" | aimd

# 代码教学
echo "用Python写一个冒泡排序，并详细解释每一步" | aimd
``` 