# aimd 测试文档

本项目包含了对aimd库所有主要功能的全面测试。

## 测试覆盖范围

### 总计：23个测试用例，100%通过

## 测试分类

### 1. 语言检测测试 (4个测试)
- `test_detect_language_chinese` - 测试通过LANG环境变量检测中文
- `test_detect_language_english` - 测试通过LANG环境变量检测英文
- `test_detect_language_lc_all_chinese` - 测试通过LC_ALL环境变量检测中文
- `test_detect_language_lc_messages_chinese` - 测试通过LC_MESSAGES环境变量检测中文

### 2. MarkdownRenderer结构体测试 (1个测试)
- `test_markdown_renderer_new` - 测试MarkdownRenderer的初始化

### 3. Markdown格式检测测试 (4个测试)
- `test_is_horizontal_rule` - 测试水平分割线的识别
- `test_find_format_candidates` - 测试格式标记候选的查找
- `test_find_matching_backticks` - 测试反引号匹配算法
- `test_apply_code_only_formatting` - 测试仅代码格式化

### 4. 格式化功能测试 (2个测试)
- `test_apply_nested_formatting` - 测试嵌套格式化（粗体内的斜体和代码）
- `test_apply_inline_formatting` - 测试内联格式化（粗体、斜体、粗斜体、代码）

### 5. 行渲染测试 (6个测试)
- `test_render_line_headers` - 测试各级标题的渲染
- `test_render_line_code_blocks` - 测试代码块的渲染
- `test_render_line_lists` - 测试列表的渲染
- `test_render_line_quotes` - 测试引用的渲染
- `test_render_line_horizontal_rules` - 测试水平分割线的渲染
- `test_render_line_plain_text` - 测试普通文本的渲染

### 6. 常量和配置测试 (2个测试)
- `test_ansi_constants` - 测试ANSI颜色常量
- `test_language_constants` - 测试语言配置常量

### 7. 命令行参数测试 (1个测试)
- `test_parse_args_mock` - 测试命令行参数解析函数

### 8. 集成测试 (1个测试)
- `test_complete_markdown_rendering` - 测试完整的Markdown文档渲染流程

### 9. 边界和性能测试 (2个测试)
- `test_edge_cases` - 测试边界情况和异常输入
- `test_performance_large_input` - 测试大输入的性能

## 测试特点

### 全面覆盖
- **函数覆盖**: 覆盖了所有公共函数和方法
- **场景覆盖**: 包括正常情况、边界情况和错误情况
- **格式覆盖**: 测试了所有支持的Markdown格式

### 测试类型
- **单元测试**: 测试单个函数的功能
- **集成测试**: 测试多个组件协同工作
- **边界测试**: 测试极端情况和错误输入
- **性能测试**: 测试大数据量的处理能力

### 安全性
- 使用`unsafe`块来修改环境变量
- 测试后恢复原始环境变量状态
- 处理并发环境下的环境变量修改

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_detect_language_chinese

# 运行特定模块的测试
cargo test tests::test_markdown_renderer_new
```

## 测试结果
```
running 23 tests
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

所有测试用例都成功通过，确保了aimd库的稳定性和可靠性。 