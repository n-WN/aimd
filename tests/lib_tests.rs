use aimd::*;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    // 测试语言检测函数
    #[test]
    fn test_detect_language_chinese() {
        // 保存原始环境变量
        let original_lang = env::var("LANG").ok();
        let original_lc_all = env::var("LC_ALL").ok();
        let original_lc_messages = env::var("LC_MESSAGES").ok();

        // 设置中文环境
        unsafe {
            env::set_var("LANG", "zh_CN.UTF-8");
        }
        let lang = detect_language();
        assert_eq!(lang.title, "aimd - AI流式Markdown渲染器");

        // 恢复原始环境变量
        unsafe {
            if let Some(val) = original_lang {
                env::set_var("LANG", val);
            } else {
                env::remove_var("LANG");
            }
            if let Some(val) = original_lc_all {
                env::set_var("LC_ALL", val);
            } else {
                env::remove_var("LC_ALL");
            }
            if let Some(val) = original_lc_messages {
                env::set_var("LC_MESSAGES", val);
            } else {
                env::remove_var("LC_MESSAGES");
            }
        }
    }

    #[test]
    fn test_detect_language_english() {
        // 保存原始环境变量
        let original_lang = env::var("LANG").ok();
        let original_lc_all = env::var("LC_ALL").ok();
        let original_lc_messages = env::var("LC_MESSAGES").ok();

        // 设置英文环境
        unsafe {
            env::set_var("LANG", "en_US.UTF-8");
            env::remove_var("LC_ALL");
            env::remove_var("LC_MESSAGES");
        }
        
        let lang = detect_language();
        assert_eq!(lang.title, "aimd - AI Streaming Markdown Renderer");

        // 恢复原始环境变量
        unsafe {
            if let Some(val) = original_lang {
                env::set_var("LANG", val);
            } else {
                env::remove_var("LANG");
            }
            if let Some(val) = original_lc_all {
                env::set_var("LC_ALL", val);
            }
            if let Some(val) = original_lc_messages {
                env::set_var("LC_MESSAGES", val);
            }
        }
    }

    #[test]
    fn test_detect_language_lc_all_chinese() {
        // 保存原始环境变量
        let original_lc_all = env::var("LC_ALL").ok();

        // 通过LC_ALL设置中文
        unsafe {
            env::set_var("LC_ALL", "zh_TW.UTF-8");
        }
        let lang = detect_language();
        assert_eq!(lang.title, "aimd - AI流式Markdown渲染器");

        // 恢复原始环境变量
        unsafe {
            if let Some(val) = original_lc_all {
                env::set_var("LC_ALL", val);
            } else {
                env::remove_var("LC_ALL");
            }
        }
    }

    #[test]
    fn test_detect_language_lc_messages_chinese() {
        // 保存原始环境变量
        let original_lc_messages = env::var("LC_MESSAGES").ok();

        // 通过LC_MESSAGES设置中文
        unsafe {
            env::set_var("LC_MESSAGES", "zh_CN.UTF-8");
        }
        let lang = detect_language();
        assert_eq!(lang.title, "aimd - AI流式Markdown渲染器");

        // 恢复原始环境变量
        unsafe {
            if let Some(val) = original_lc_messages {
                env::set_var("LC_MESSAGES", val);
            } else {
                env::remove_var("LC_MESSAGES");
            }
        }
    }

    // 测试MarkdownRenderer的创建
    #[test]
    fn test_markdown_renderer_new() {
        let renderer = MarkdownRenderer::new();
        assert!(!renderer.in_code_block);
        assert!(!renderer.in_list);
        assert!(renderer.code_lang.is_empty());
    }

    // 测试水平分割线识别
    #[test]
    fn test_is_horizontal_rule() {
        let renderer = MarkdownRenderer::new();
        
        // 测试有效的水平分割线
        assert!(renderer.is_horizontal_rule("---"));
        assert!(renderer.is_horizontal_rule("***"));
        assert!(renderer.is_horizontal_rule("___"));
        assert!(renderer.is_horizontal_rule("----"));
        assert!(renderer.is_horizontal_rule("*****"));
        assert!(renderer.is_horizontal_rule("______"));
        
        // 测试无效的水平分割线
        assert!(!renderer.is_horizontal_rule("--"));
        assert!(!renderer.is_horizontal_rule("**"));
        assert!(!renderer.is_horizontal_rule("__"));
        assert!(!renderer.is_horizontal_rule(""));
        assert!(!renderer.is_horizontal_rule("abc"));
        assert!(!renderer.is_horizontal_rule("--*"));
        assert!(!renderer.is_horizontal_rule("***---"));
    }

    // 测试格式候选查找
    #[test]
    fn test_find_format_candidates() {
        let renderer = MarkdownRenderer::new();
        
        // 测试粗斜体格式
        let candidates = renderer.find_format_candidates("***bold italic***", 0);
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|(_, format_type, _)| format_type == "bold_italic_star"));
        
        // 测试粗体格式
        let candidates = renderer.find_format_candidates("**bold text**", 0);
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|(_, format_type, _)| format_type == "bold_star"));
        
        // 测试斜体格式
        let candidates = renderer.find_format_candidates("*italic text*", 0);
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|(_, format_type, _)| format_type == "italic_star"));
        
        // 测试行内代码
        let candidates = renderer.find_format_candidates("`code`", 0);
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|(_, format_type, _)| format_type == "code"));
        
        // 测试下划线格式
        let candidates = renderer.find_format_candidates("___bold italic___", 0);
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|(_, format_type, _)| format_type == "bold_italic_underscore"));
        
        let candidates = renderer.find_format_candidates("__bold__", 0);
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|(_, format_type, _)| format_type == "bold_underscore"));
        
        let candidates = renderer.find_format_candidates("_italic_", 0);
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|(_, format_type, _)| format_type == "italic_underscore"));
    }

    // 测试反引号匹配
    #[test]
    fn test_find_matching_backticks() {
        let renderer = MarkdownRenderer::new();
        
        // 测试单个反引号
        assert_eq!(renderer.find_matching_backticks("`code`", 1), Some(5));
        assert_eq!(renderer.find_matching_backticks("`hello world`", 1), Some(12));
        
        // 测试双重反引号
        assert_eq!(renderer.find_matching_backticks("``code``", 2), Some(6));
        
        // 测试不匹配的反引号
        assert_eq!(renderer.find_matching_backticks("`code", 1), None);
        
        // 测试带有单个反引号的三重反引号字符串
        // 对于 "```code```"，第一个反引号开始，我们寻找下一个单独的反引号
        // 从"``code```"开始寻找，第一个反引号实际上是位置0，但是连续有2个
        // 由于2 >= 1，函数返回位置0+1=1，这是正确的
        assert_eq!(renderer.find_matching_backticks("```code```", 1), Some(1));
        
        // 测试更清晰的情况：找到code后的第一个反引号
        assert_eq!(renderer.find_matching_backticks("`code```", 1), Some(5));
    }

    // 测试仅代码格式化
    #[test]
    fn test_apply_code_only_formatting() {
        let renderer = MarkdownRenderer::new();
        
        let result = renderer.apply_code_only_formatting("text with `code` inside");
        assert!(result.contains(LIGHT_GRAY_BG));
        assert!(result.contains(DARK_ORANGE_RED));
        assert!(result.contains("code"));
        
        let result = renderer.apply_code_only_formatting("no code here");
        assert_eq!(result, "no code here");
        
        let result = renderer.apply_code_only_formatting("`code1` and `code2`");
        assert!(result.contains(LIGHT_GRAY_BG));
        assert!(result.contains("code1"));
        assert!(result.contains("code2"));
    }

    // 测试嵌套格式化
    #[test]
    fn test_apply_nested_formatting() {
        let renderer = MarkdownRenderer::new();
        
        // 测试在粗体内部的斜体
        let result = renderer.apply_nested_formatting("bold with *italic* text");
        assert!(result.contains(ITALIC));
        assert!(result.contains("italic"));
        
        // 测试在粗体内部的代码
        let result = renderer.apply_nested_formatting("bold with `code` text");
        assert!(result.contains(LIGHT_GRAY_BG));
        assert!(result.contains("code"));
        
        // 测试普通文本
        let result = renderer.apply_nested_formatting("just bold text");
        assert_eq!(result, "just bold text");
    }

    // 测试内联格式化
    #[test]
    fn test_apply_inline_formatting() {
        let renderer = MarkdownRenderer::new();
        
        // 测试粗体
        let result = renderer.apply_inline_formatting("**bold text**");
        assert!(result.contains(BOLD));
        assert!(result.contains("bold text"));
        assert!(result.contains(RESET));
        
        // 测试斜体
        let result = renderer.apply_inline_formatting("*italic text*");
        assert!(result.contains(ITALIC));
        assert!(result.contains("italic text"));
        
        // 测试粗斜体
        let result = renderer.apply_inline_formatting("***bold italic***");
        assert!(result.contains(BOLD_ITALIC));
        assert!(result.contains("bold italic"));
        
        // 测试行内代码
        let result = renderer.apply_inline_formatting("`code`");
        assert!(result.contains(LIGHT_GRAY_BG));
        assert!(result.contains(DARK_ORANGE_RED));
        assert!(result.contains("code"));
        
        // 测试下划线格式
        let result = renderer.apply_inline_formatting("__bold text__");
        assert!(result.contains(BOLD));
        assert!(result.contains("bold text"));
        
        let result = renderer.apply_inline_formatting("_italic text_");
        assert!(result.contains(ITALIC));
        assert!(result.contains("italic text"));
        
        let result = renderer.apply_inline_formatting("___bold italic___");
        assert!(result.contains(BOLD_ITALIC));
        assert!(result.contains("bold italic"));
        
        // 测试普通文本
        let result = renderer.apply_inline_formatting("plain text");
        assert_eq!(result, "plain text");
    }

    // 测试行渲染 - 标题
    #[test]
    fn test_render_line_headers() {
        let mut renderer = MarkdownRenderer::new();
        
        // 测试一级标题
        let result = renderer.render_line("# Header 1");
        assert!(result.contains(CYAN));
        assert!(result.contains("━━"));
        assert!(result.contains("Header 1"));
        
        // 测试二级标题
        let result = renderer.render_line("## Header 2");
        assert!(result.contains("──"));
        assert!(result.contains("Header 2"));
        
        // 测试三级标题
        let result = renderer.render_line("### Header 3");
        assert!(result.contains("▸"));
        assert!(result.contains("Header 3"));
        
        // 测试四级标题
        let result = renderer.render_line("#### Header 4");
        assert!(result.contains("•"));
        assert!(result.contains("Header 4"));
        
        // 测试五级标题
        let result = renderer.render_line("##### Header 5");
        assert!(result.contains("‣"));
        assert!(result.contains("Header 5"));
        
        // 测试六级标题
        let result = renderer.render_line("###### Header 6");
        assert!(result.contains("◦"));
        assert!(result.contains("Header 6"));
    }

    // 测试行渲染 - 代码块
    #[test]
    fn test_render_line_code_blocks() {
        let mut renderer = MarkdownRenderer::new();
        
        // 测试代码块开始
        let result = renderer.render_line("```");
        assert!(result.contains("┌─ 代码块开始"));
        assert!(renderer.in_code_block);
        
        // 测试代码块内容
        let result = renderer.render_line("let x = 42;");
        assert!(result.contains(GREEN));
        assert!(result.contains("let x = 42;"));
        
        // 测试代码块结束
        let result = renderer.render_line("```");
        assert!(result.contains("└─ 代码块结束"));
        assert!(!renderer.in_code_block);
        
        // 测试带语言标识的代码块
        let mut renderer2 = MarkdownRenderer::new();
        let result = renderer2.render_line("```rust");
        assert!(result.contains("┌─ 代码块开始 rust"));
        assert!(renderer2.in_code_block);
        assert_eq!(renderer2.code_lang, "rust");
    }

    // 测试行渲染 - 列表
    #[test]
    fn test_render_line_lists() {
        let mut renderer = MarkdownRenderer::new();
        
        // 测试破折号列表
        let result = renderer.render_line("- List item 1");
        assert!(result.contains(MAGENTA));
        assert!(result.contains("•"));
        assert!(result.contains("List item 1"));
        assert!(renderer.in_list);
        
        // 测试星号列表
        let result = renderer.render_line("* List item 2");
        assert!(result.contains("•"));
        assert!(result.contains("List item 2"));
        
        // 测试缩进列表
        let result = renderer.render_line("    - Indented item");
        assert!(result.contains("  "));  // 缩进
        assert!(result.contains("Indented item"));
        
        // 测试空列表项
        let result = renderer.render_line("-");
        assert!(result.contains("•"));
        
        let result = renderer.render_line("*");
        assert!(result.contains("•"));
    }

    // 测试行渲染 - 引用
    #[test]
    fn test_render_line_quotes() {
        let mut renderer = MarkdownRenderer::new();
        
        let result = renderer.render_line("> This is a quote");
        assert!(result.contains(YELLOW));
        assert!(result.contains("│"));
        assert!(result.contains("This is a quote"));
        
        let result = renderer.render_line(">Another quote");
        assert!(result.contains("│"));
        assert!(result.contains("Another quote"));
    }

    // 测试行渲染 - 水平分割线
    #[test]
    fn test_render_line_horizontal_rules() {
        let mut renderer = MarkdownRenderer::new();
        
        let result = renderer.render_line("---");
        assert!(result.contains(GRAY));
        assert!(result.contains(&"─".repeat(7)));
        
        let result = renderer.render_line("***");
        assert!(result.contains(&"─".repeat(7)));
        
        let result = renderer.render_line("___");
        assert!(result.contains(&"─".repeat(7)));
    }

    // 测试行渲染 - 普通文本
    #[test]
    fn test_render_line_plain_text() {
        let mut renderer = MarkdownRenderer::new();
        
        let result = renderer.render_line("This is plain text");
        assert_eq!(result, "This is plain text\n");
        
        // 测试包含格式的普通文本
        let result = renderer.render_line("Text with **bold** and *italic*");
        assert!(result.contains(BOLD));
        assert!(result.contains(ITALIC));
        assert!(result.contains("bold"));
        assert!(result.contains("italic"));
    }

    // 测试parse_args函数 - 需要模拟命令行参数
    #[test]
    fn test_parse_args_mock() {
        // 注意：parse_args函数使用std::env::args()，这在测试环境中很难完全模拟
        // 这里我们只能测试函数存在并返回正确的类型
        
        // 这个测试主要验证函数签名和返回类型
        let _result: (Option<String>, Option<Vec<String>>, bool) = parse_args();
        // 由于parse_args依赖于真实的命令行参数，我们只验证它能被调用
    }

    // 测试Language结构体的字段
    #[test]
    fn test_language_constants() {
        // 测试中文语言常量
        assert_eq!(CHINESE.title, "aimd - AI流式Markdown渲染器");
        assert_eq!(CHINESE.usage, "用法:");
        assert_eq!(CHINESE.options, "选项:");
        assert!(CHINESE.help.contains("帮助"));
        
        // 测试英文语言常量
        assert_eq!(ENGLISH.title, "aimd - AI Streaming Markdown Renderer");
        assert_eq!(ENGLISH.usage, "Usage:");
        assert_eq!(ENGLISH.options, "Options:");
        assert!(ENGLISH.help.contains("help"));
    }

    // 测试ANSI颜色常量
    #[test]
    fn test_ansi_constants() {
        assert_eq!(RESET, "\x1b[0m");
        assert_eq!(BOLD, "\x1b[1;33m");
        assert_eq!(ITALIC, "\x1b[3m");
        assert_eq!(BOLD_ITALIC, "\x1b[1;3;33m");
        assert_eq!(CYAN, "\x1b[36m");
        assert_eq!(GREEN, "\x1b[32m");
        assert_eq!(YELLOW, "\x1b[33m");
        assert_eq!(MAGENTA, "\x1b[35m");
        assert_eq!(LIGHT_GRAY_BG, "\x1b[48;5;253m");
        assert_eq!(DARK_ORANGE_RED, "\x1b[38;5;130m");
        assert_eq!(GRAY, "\x1b[90m");
    }

    // 集成测试 - 测试完整的markdown渲染流程
    #[test]
    fn test_complete_markdown_rendering() {
        let mut renderer = MarkdownRenderer::new();
        
        // 测试多行markdown文档
        let lines = vec![
            "# Main Title",
            "",
            "This is **bold** and *italic* text.",
            "",
            "```rust",
            "fn main() {",
            "    println!(\"Hello, world!\");",
            "}",
            "```",
            "",
            "- First item",
            "- Second item with `code`",
            "",
            "> This is a quote",
            "",
            "---",
        ];
        
        let mut results = Vec::new();
        for line in lines {
            results.push(renderer.render_line(line));
        }
        
        // 验证标题被正确渲染
        assert!(results[0].contains(CYAN));
        assert!(results[0].contains("━━"));
        assert!(results[0].contains("Main Title"));
        
        // 验证格式化文本
        assert!(results[2].contains(BOLD));
        assert!(results[2].contains(ITALIC));
        
        // 验证代码块
        assert!(results[4].contains("┌─ 代码块开始 rust"));
        assert!(results[5].contains(GREEN));
        assert!(results[8].contains("└─ 代码块结束"));
        
        // 验证列表
        assert!(results[10].contains(MAGENTA));
        assert!(results[10].contains("•"));
        assert!(results[11].contains("code"));
        
        // 验证引用
        assert!(results[13].contains(YELLOW));
        assert!(results[13].contains("│"));
        
        // 验证水平分割线
        assert!(results[15].contains(GRAY));
        assert!(results[15].contains(&"─".repeat(7)));
    }

    // 边界测试
    #[test]
    fn test_edge_cases() {
        let mut renderer = MarkdownRenderer::new();
        
        // 测试空字符串
        let result = renderer.render_line("");
        assert_eq!(result, "\n");
        
        // 测试只有空格的字符串
        let result = renderer.render_line("   ");
        assert_eq!(result, "   \n");
        
        // 测试不完整的格式标记
        let result = renderer.render_line("**incomplete bold");
        assert!(result.contains("**incomplete bold"));
        
        let result = renderer.render_line("*incomplete italic");
        assert!(result.contains("*incomplete italic"));
        
        let result = renderer.render_line("`incomplete code");
        assert!(result.contains("`incomplete code"));
        
        // 测试嵌套的未闭合标记
        let result = renderer.render_line("**bold with *italic** incomplete");
        assert!(result.contains(BOLD));
        
        // 测试很长的水平分割线
        let result = renderer.render_line(&"-".repeat(100));
        assert!(result.contains(&"─".repeat(7)));
        
        // 测试混合的分割线字符（应该不被识别）
        let result = renderer.render_line("---***");
        assert!(!result.contains(&"─".repeat(7)));
    }

    // 性能相关测试
    #[test]
    fn test_performance_large_input() {
        let mut renderer = MarkdownRenderer::new();
        
        // 测试处理大文本的性能
        let large_text = "This is a very long line of text that contains **bold**, *italic*, and `code` elements. ".repeat(100);
        let result = renderer.render_line(&large_text);
        
        // 验证所有格式都被正确处理
        assert!(result.contains(BOLD));
        assert!(result.contains(ITALIC));
        assert!(result.contains(LIGHT_GRAY_BG));
        
        // 测试大量反引号的处理
        let backtick_text = "`".repeat(1000);
        let result = renderer.render_line(&backtick_text);
        // 应该不会崩溃，并且返回某种结果
        assert!(!result.is_empty());
    }
} 