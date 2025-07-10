use std::error::Error;
use std::io::{Write, BufRead, BufReader, IsTerminal, Read};
use std::env;
// use std::process::{Command, Stdio};
// We only need Command and open from the blocking module.
use pty_process::blocking::{open, Command as PtyCommand};

// ANSI 颜色常量
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1;33m";  // 黄色粗体，更明显
const ITALIC: &str = "\x1b[3m";   // 斜体
const BOLD_ITALIC: &str = "\x1b[1;3;33m";  // 粗斜体（粗体+斜体+黄色）
const CYAN: &str = "\x1b[36m";      // 标题
const GREEN: &str = "\x1b[32m";     // 代码
const YELLOW: &str = "\x1b[33m";    // 引用
// const BLUE: &str = "\x1b[34m";      // 链接
// const RED: &str = "\x1b[31m";       // 强调
const MAGENTA: &str = "\x1b[35m";   // 列表项
const LIGHT_GRAY_BG: &str = "\x1b[48;5;253m";  // 更淡的灰色背景
const DARK_ORANGE_RED: &str = "\x1b[38;5;130m";  // 偏橘色的暗红色文字
const GRAY: &str = "\x1b[90m";      // 灰色文字（用于水平分割线）

// 语言配置
struct Language {
    title: &'static str,
    usage: &'static str,
    options: &'static str,
    help: &'static str,
    separator: &'static str,
    description: &'static str,
    examples: &'static str,
    show_help: &'static str,
    pipe_input: &'static str,
    explicit_cmd: &'static str,
    debug_mode: &'static str,
    preparing: &'static str,
    started: &'static str,
    completed: &'static str,
    exited: &'static str,
    debug_enabled: &'static str,
    command: &'static str,
    args: &'static str,
    pipe_input_detected: &'static str,
    reading_next_line: &'static str,
    bytes_read: &'static str,
    read_finished: &'static str,
    starting_render: &'static str,
    render_completed: &'static str,
    unknown_option: &'static str,
    use_help: &'static str,
    error_separator_needs_program: &'static str,
    // error_program_needs_name: &'static str,
}

const CHINESE: Language = Language {
    title: "aimd - AI流式Markdown渲染器",
    usage: "用法:",
    options: "选项:",
    help: "显示此帮助信息",
    separator: "分隔符，后面的所有参数都传递给AI程序",
    description: "说明:",
    examples: "示例:",
    show_help: "显示帮助",
    pipe_input: "管道输入",
    explicit_cmd: "显式命令",
    debug_mode: "调试模式",
    preparing: "准备在 PTY 环境中启动",
    started: "进程已启动，开始流式 Markdown 渲染...",
    completed: "流式渲染完成！",
    exited: "进程已退出:",
    debug_enabled: "[DEBUG] 调试模式已启用",
    command: "[DEBUG] 命令:",
    args: "[DEBUG] 参数:",
    pipe_input_detected: "[DEBUG] 从管道读取到输入:",
    reading_next_line: "[DEBUG] 准备读取下一行...",
    bytes_read: "[DEBUG] 读取 {} 字节:",
    read_finished: "[DEBUG] 读取结束",
    starting_render: "[DEBUG] 开始渲染...",
    render_completed: "[DEBUG] 渲染完成，输出长度:",
    unknown_option: "未知选项:",
    use_help: "使用 --help 查看帮助信息",
    error_separator_needs_program: "错误: -- 后面需要指定程序名称",
    // error_program_needs_name: "错误: --program 需要指定程序名称",
};

const ENGLISH: Language = Language {
    title: "aimd - AI Streaming Markdown Renderer",
    usage: "Usage:",
    options: "Options:",
    help: "Show this help message",
    separator: "Separator, all subsequent arguments are passed to AI program",
    description: "Description:",
    examples: "Examples:",
    show_help: "Show help",
    pipe_input: "Pipe input",
    explicit_cmd: "Explicit command",
    debug_mode: "Debug mode",
    preparing: "Preparing to start in PTY environment:",
    started: "process started, beginning streaming Markdown rendering...",
    completed: "Streaming rendering completed!",
    exited: "process exited:",
    debug_enabled: "[DEBUG] Debug mode enabled",
    command: "[DEBUG] Command:",
    args: "[DEBUG] Args:",
    pipe_input_detected: "[DEBUG] Pipe input detected:",
    reading_next_line: "[DEBUG] Reading next line...",
    bytes_read: "[DEBUG] Read {} bytes:",
    read_finished: "[DEBUG] Read finished",
    starting_render: "[DEBUG] Starting render...",
    render_completed: "[DEBUG] Render completed, output length:",
    unknown_option: "Unknown option:",
    use_help: "Use --help to see help information",
    error_separator_needs_program: "Error: Program name required after --",
    // error_program_needs_name: "Error: --program requires program name",
};

fn detect_language() -> &'static Language {
    if let Ok(lang) = env::var("LANG") {
        if lang.starts_with("zh") {
            return &CHINESE;
        }
    }
    if let Ok(lang) = env::var("LC_ALL") {
        if lang.starts_with("zh") {
            return &CHINESE;
        }
    }
    if let Ok(lang) = env::var("LC_MESSAGES") {
        if lang.starts_with("zh") {
            return &CHINESE;
        }
    }
    &ENGLISH
}

struct MarkdownRenderer {
    in_code_block: bool,
    in_list: bool,
    code_lang: String,
}

impl MarkdownRenderer {
    fn new() -> Self {
        Self {
            in_code_block: false,
            in_list: false,
            code_lang: String::new(),
        }
    }

    fn apply_inline_formatting(&self, text: &str) -> String {
        // 完整的markdown格式支持：粗体、斜体、粗斜体、行内代码（星号和下划线）
        let mut result = String::new();
        let mut pos = 0;
        
        while pos < text.len() {
            // 寻找下一个格式标记（按优先级）
            let candidates = self.find_format_candidates(&text[pos..], pos);
            
            if let Some((start, format_type, marker_len)) = candidates.into_iter().min_by_key(|&(pos, _, _)| pos) {
                // 添加格式标记之前的文本
                result.push_str(&text[pos..start]);
                
                match format_type.as_str() {
                    "bold_italic_star" => {
                        // ***text*** 粗斜体
                        if let Some(end) = text[start + 3..].find("***") {
                            let end = end + start + 3;
                            let content = &text[start + 3..end];
                            let formatted_content = self.apply_code_only_formatting(content);
                            result.push_str(&format!("{}{}{}", BOLD_ITALIC, formatted_content, RESET));
                            pos = end + 3;
                        } else {
                            result.push_str(&text[start..start + marker_len]);
                            pos = start + marker_len;
                        }
                    }
                    "bold_italic_underscore" => {
                        // ___text___ 粗斜体
                        if let Some(end) = text[start + 3..].find("___") {
                            let end = end + start + 3;
                            let content = &text[start + 3..end];
                            let formatted_content = self.apply_code_only_formatting(content);
                            result.push_str(&format!("{}{}{}", BOLD_ITALIC, formatted_content, RESET));
                            pos = end + 3;
                        } else {
                            result.push_str(&text[start..start + marker_len]);
                            pos = start + marker_len;
                        }
                    }
                    "bold_star" => {
                        // **text** 粗体
                        if let Some(end) = text[start + 2..].find("**") {
                            let end = end + start + 2;
                            let content = &text[start + 2..end];
                            let formatted_content = self.apply_nested_formatting(content);
                            result.push_str(&format!("{}{}{}", BOLD, formatted_content, RESET));
                            pos = end + 2;
                        } else {
                            result.push_str(&text[start..start + marker_len]);
                            pos = start + marker_len;
                        }
                    }
                    "bold_underscore" => {
                        // __text__ 粗体
                        if let Some(end) = text[start + 2..].find("__") {
                            let end = end + start + 2;
                            let content = &text[start + 2..end];
                            let formatted_content = self.apply_nested_formatting(content);
                            result.push_str(&format!("{}{}{}", BOLD, formatted_content, RESET));
                            pos = end + 2;
                        } else {
                            result.push_str(&text[start..start + marker_len]);
                            pos = start + marker_len;
                        }
                    }
                    "italic_star" => {
                        // *text* 斜体
                        if let Some(end) = text[start + 1..].find('*') {
                            let end = end + start + 1;
                            let content = &text[start + 1..end];
                            let formatted_content = self.apply_code_only_formatting(content);
                            result.push_str(&format!("{}{}{}", ITALIC, formatted_content, RESET));
                            pos = end + 1;
                        } else {
                            result.push_str(&text[start..start + marker_len]);
                            pos = start + marker_len;
                        }
                    }
                    "italic_underscore" => {
                        // _text_ 斜体
                        if let Some(end) = text[start + 1..].find('_') {
                            let end = end + start + 1;
                            let content = &text[start + 1..end];
                            let formatted_content = self.apply_code_only_formatting(content);
                            result.push_str(&format!("{}{}{}", ITALIC, formatted_content, RESET));
                            pos = end + 1;
                        } else {
                            result.push_str(&text[start..start + marker_len]);
                            pos = start + marker_len;
                        }
                    }
                    "code" => {
                        // `code` 行内代码
                        let remaining = &text[start..];
                        let backtick_count = remaining.chars().take_while(|&c| c == '`').count();
                        
                        if backtick_count >= 3 {
                            result.push_str(&text[start..start + backtick_count]);
                            pos = start + backtick_count;
                        } else if let Some(end) = self.find_matching_backticks(&text[start..], backtick_count) {
                            let end = end + start;
                            let content = &text[start + backtick_count..end];
                            result.push_str(&format!("{}{} {} {}", LIGHT_GRAY_BG, DARK_ORANGE_RED, content, RESET));
                            pos = end + backtick_count;
                        } else {
                            result.push_str(&text[start..start + 1]);
                            pos = start + 1;
                        }
                    }
                    _ => {
                        result.push_str(&text[start..start + 1]);
                        pos = start + 1;
                    }
                }
            } else {
                // 没有更多格式标记，添加剩余文本
                result.push_str(&text[pos..]);
                break;
            }
        }
        
        result
    }
    
    fn find_format_candidates(&self, text: &str, offset: usize) -> Vec<(usize, String, usize)> {
        let mut candidates = Vec::new();
        
        // 查找各种格式标记（按优先级：长的优先）
        if let Some(pos) = text.find("***") {
            candidates.push((pos + offset, "bold_italic_star".to_string(), 3));
        }
        if let Some(pos) = text.find("___") {
            candidates.push((pos + offset, "bold_italic_underscore".to_string(), 3));
        }
        if let Some(pos) = text.find("**") {
            candidates.push((pos + offset, "bold_star".to_string(), 2));
        }
        if let Some(pos) = text.find("__") {
            candidates.push((pos + offset, "bold_underscore".to_string(), 2));
        }
        if let Some(pos) = text.find('*') {
            // 确保这个*不是**或***的一部分
            if !text[pos..].starts_with("**") {
                candidates.push((pos + offset, "italic_star".to_string(), 1));
            }
        }
        if let Some(pos) = text.find('_') {
            // 确保这个_不是__或___的一部分
            if !text[pos..].starts_with("__") {
                candidates.push((pos + offset, "italic_underscore".to_string(), 1));
            }
        }
        if let Some(pos) = text.find('`') {
            candidates.push((pos + offset, "code".to_string(), 1));
        }
        
        candidates
    }
    
    fn find_matching_backticks(&self, text: &str, count: usize) -> Option<usize> {
        let search_text = &text[count..];
        let mut search_pos = 0;
        
        while let Some(candidate_start) = search_text[search_pos..].find('`') {
            let candidate_pos = candidate_start + search_pos;
            let candidate_remaining = &search_text[candidate_pos..];
            let candidate_count = candidate_remaining.chars().take_while(|&c| c == '`').count();
            
            if candidate_count >= count {
                return Some(candidate_pos + count);
            } else {
                search_pos = candidate_pos + 1;
            }
        }
        
        None
    }
    
    fn apply_nested_formatting(&self, text: &str) -> String {
        // 在粗体内部，只处理斜体和行内代码
        let mut result = String::new();
        let mut pos = 0;
        
        while pos < text.len() {
            let next_italic = text[pos..].find('*').map(|i| (i + pos, "italic"));
            let next_code = text[pos..].find('`').map(|i| (i + pos, "code"));
            
            let next_format = match (next_italic, next_code) {
                (Some(italic), Some(code)) => {
                    if italic.0 < code.0 { Some(italic) } else { Some(code) }
                }
                (Some(italic), None) => Some(italic),
                (None, Some(code)) => Some(code),
                (None, None) => None,
            };
            
            match next_format {
                None => {
                    result.push_str(&text[pos..]);
                    break;
                }
                Some((start, "italic")) => {
                    if let Some(end_offset) = text[start + 1..].find('*') {
                        let end = end_offset + start + 1;
                        let italic_content = &text[start + 1..end];
                        
                        result.push_str(&text[pos..start]);
                        result.push_str(&format!("{}{}{}", ITALIC, italic_content, RESET));
                        pos = end + 1;
                    } else {
                        result.push_str(&text[pos..start + 1]);
                        pos = start + 1;
                    }
                }
                Some((start, "code")) => {
                    if let Some(end_offset) = text[start + 1..].find('`') {
                        let end = end_offset + start + 1;
                        let code_content = &text[start + 1..end];
                        
                        result.push_str(&text[pos..start]);
                        result.push_str(&format!("{}{} {} {}", LIGHT_GRAY_BG, DARK_ORANGE_RED, code_content, RESET));
                        pos = end + 1;
                    } else {
                        result.push_str(&text[pos..start + 1]);
                        pos = start + 1;
                    }
                }
                _ => pos += 1,
            }
        }
        
        result
    }
    
    fn apply_code_only_formatting(&self, text: &str) -> String {
        // 在斜体内部，只处理行内代码
        let mut result = text.to_string();
        let mut pos = 0;
        
        while let Some(start) = result[pos..].find('`') {
            let start = start + pos;
            if let Some(end) = result[start + 1..].find('`') {
                let end = end + start + 1;
                let code_content = &result[start + 1..end];
                let formatted = format!("{}{} {} {}", LIGHT_GRAY_BG, DARK_ORANGE_RED, code_content, RESET);
                result.replace_range(start..end + 1, &formatted);
                pos = start + formatted.len();
            } else {
                pos = start + 1;
            }
        }
        
        result
    }
    


    fn is_horizontal_rule(&self, trimmed: &str) -> bool {
        // 检查是否为水平分割线
        // 支持: ---, ***, ___, 以及它们的更长版本
        if trimmed.len() < 3 {
            return false;
        }
        
        // 检查是否全部由相同的分割线字符组成
        let chars: Vec<char> = trimmed.chars().collect();
        let first_char = chars[0];
        
        if first_char == '-' || first_char == '*' || first_char == '_' {
            // 检查是否全部都是同一个字符
            chars.iter().all(|&c| c == first_char) && chars.len() >= 3
        } else {
            false
        }
    }

    fn render_line(&mut self, line: &str) -> String {
        let trimmed = line.trim();
        
        // 处理代码块
        if trimmed.starts_with("```") && trimmed.len() == 3 {
            // 只有恰好3个反引号才作为代码块边界处理
            if self.in_code_block {
                self.in_code_block = false;
                self.code_lang.clear();
                return format!("{}{}└─ 代码块结束{}\n", 
                    GREEN, BOLD, RESET);
            } else {
                self.in_code_block = true;
                return format!("{}{}┌─ 代码块开始{}\n", 
                    GREEN, BOLD, RESET);
            }
        } else if trimmed.starts_with("```") && !self.in_code_block {
            // 如果不在代码块内，且是```后跟语言标识，开始代码块
            self.in_code_block = true;
            if trimmed.len() > 3 {
                self.code_lang = trimmed[3..].to_string();
            }
            return format!("{}{}┌─ 代码块开始 {}{}\n", 
                GREEN, BOLD, &self.code_lang, RESET);
        }

        // 在代码块内部
        if self.in_code_block {
            return format!("{}{}", GREEN, line);
        }

        // 处理标题
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            let title = trimmed.trim_start_matches('#').trim();
            let formatted_title = self.apply_inline_formatting(title);
            let prefix = match level {
                1 => format!("{}{}━━ ", CYAN, BOLD),
                2 => format!("{}{}── ", CYAN, BOLD),
                3 => format!("{}{}▸ ", CYAN, BOLD),
                4 => format!("{}{}• ", CYAN, BOLD),
                5 => format!("{}{}‣ ", CYAN, BOLD),
                _ => format!("{}{}◦ ", CYAN, BOLD),
            };
            return format!("{}{}{}{}\n", prefix, formatted_title, RESET, RESET);
        }

        // 处理水平分割线
        if self.is_horizontal_rule(trimmed) {
            return format!("{}{}{}\n", 
                GRAY, "─".repeat(7), RESET);
        }

        // 处理列表 - 必须在apply_inline_formatting之前检查
        // 改进列表检测：只有在空格后跟内容或单独的标记符才是列表
        let is_list = trimmed.starts_with("- ") || 
                     trimmed.starts_with("* ") || 
                     (trimmed == "-") || 
                     (trimmed == "*");
        
        if is_list {
            // 计算缩进级别
            let indent_level = (line.len() - line.trim_start().len()) / 4; // 每4个空格为一级
            let indent = "  ".repeat(indent_level); // 每级2个空格缩进
            
            // 更智能地提取列表内容
            let content = if trimmed.starts_with("- ") {
                &trimmed[2..]
            } else if trimmed.starts_with("* ") {
                &trimmed[2..]
            } else if trimmed == "-" || trimmed == "*" {
                ""
            } else {
                trimmed
            };
            let formatted_content = self.apply_inline_formatting(content);
            self.in_list = true;
            return format!("{}{}{} •{} {}{}\n", 
                indent, MAGENTA, BOLD, RESET, formatted_content, RESET);
        }

        // 处理普通文本中的格式
        let mut result = line.to_string();
        result = self.apply_inline_formatting(&result);

        // 处理引用
        if trimmed.starts_with('>') {
            let content = trimmed[1..].trim();
            let formatted_content = self.apply_inline_formatting(content);
            return format!("{}{}│{} {}{}\n", 
                YELLOW, BOLD, RESET, formatted_content, RESET);
        }

        format!("{}\n", result)
    }
}

fn print_help() {
    let lang = detect_language();
    let prog_name = std::env::args().next().unwrap_or("aimd".to_string());
    
    println!("{}{}{}{}{}", CYAN, BOLD, lang.title, RESET, RESET);
    println!();
    println!("{}:", lang.usage);
    println!("  {} [options] [--] <program> [program args...]", prog_name);
    println!("  echo \"prompt\" | {} [options]", prog_name);
    println!();
    println!("{}:", lang.options);
    println!("  --debug           {}", lang.debug_mode);
    println!("  --help            {}", lang.help);
    println!("  --                {}", lang.separator);
    println!();
    println!("{}:", lang.description);
    if lang as *const _ == &CHINESE as *const _ {
        println!("  1. 无参数时：显示此帮助信息");
        println!("  2. 管道输入时：将输入作为提示词传递给默认的gemini程序");
        println!("  3. 显式命令时：在 -- 之后指定AI程序和它的所有参数");
        println!("  程序的输出将通过流式Markdown渲染器显示");
    } else {
        println!("  1. No arguments: Show this help message");
        println!("  2. Pipe input: Pass input as prompt to default gemini program");
        println!("  3. Explicit command: Specify AI program and all its arguments after --");
        println!("  Program output will be displayed through streaming Markdown renderer");
    }
    println!();
    println!("{}:", lang.examples);
    println!("  {}                                               # {}", prog_name, lang.show_help);
    if lang as *const _ == &CHINESE as *const _ {
        println!("  echo \"介绍Rust\" | {}                            # {}", prog_name, lang.pipe_input);
        println!("  {} -- gemini --model gemini-2.5-flash -p \"介绍Rust\"  # {}", prog_name, lang.explicit_cmd);
        println!("  {} -- claude --model sonnet -p \"解释async/await\"     # Claude", prog_name);
        println!("  {} --debug -- gemini --help                       # {}", prog_name, lang.debug_mode);
    } else {
        println!("  echo \"Introduce Rust\" | {}                        # {}", prog_name, lang.pipe_input);
        println!("  {} -- gemini --model gemini-2.5-flash -p \"Introduce Rust\"  # {}", prog_name, lang.explicit_cmd);
        println!("  {} -- claude --model sonnet -p \"Explain async/await\"       # Claude", prog_name);
        println!("  {} --debug -- gemini --help                             # {}", prog_name, lang.debug_mode);
    }
}

fn parse_args() -> (Option<String>, Option<Vec<String>>, bool) {
    let lang = detect_language();
    let args: Vec<String> = std::env::args().collect();
    let mut debug_mode = false;
    let mut separator_found = false;
    let mut command = String::new();
    let mut command_args = Vec::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            "--debug" => {
                debug_mode = true;
            }
            "--" => {
                separator_found = true;
                // 后面的所有参数都是程序和程序参数
                if i + 1 < args.len() {
                    command = args[i + 1].clone();
                    command_args = args[i + 2..].to_vec();
                } else {
                    eprintln!("{}", lang.error_separator_needs_program);
                    std::process::exit(1);
                }
                break;
            }
            _ => {
                eprintln!("{} {}", lang.unknown_option, args[i]);
                eprintln!("{}", lang.use_help);
                std::process::exit(1);
            }
        }
        i += 1;
    }
    
    // 如果没有找到分隔符，返回 None 表示需要特殊处理
    if !separator_found {
        (None, None, debug_mode)
    } else {
        (Some(command), Some(command_args), debug_mode)
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let lang = detect_language();
    
    // --- 解析命令行参数 ---
    let (command_opt, command_args_opt, debug_mode) = parse_args();
    
    // 检查是否有管道输入或显式命令
    let (command, command_args) = match (command_opt, command_args_opt) {
        (Some(cmd), Some(args)) => {
            // 用户提供了显式命令
            (cmd, args)
        }
        (None, None) => {
            // 没有显式命令，检查是否有管道输入
            if std::io::stdin().is_terminal() {
                // 没有管道输入，显示帮助信息
                print_help();
                std::process::exit(0);
            } else {
                // 有管道输入，读取所有内容作为提示词
                let mut input = String::new();
                std::io::stdin().read_to_string(&mut input)?;
                
                if debug_mode {
                    println!("{}{}{} {:?}{}{}", CYAN, BOLD, lang.pipe_input_detected, input.trim(), RESET, RESET);
                }
                
                let command = "gemini".to_string();
                let command_args = vec![
                    "--model".to_string(),
                    "gemini-2.5-flash".to_string(),
                    "-p".to_string(),
                    input.trim().to_string()
                ];
                
                (command, command_args)
            }
        }
        _ => unreachable!(),
    };
    
    if debug_mode {
        println!("{}{}{}{}{}", CYAN, BOLD, lang.debug_enabled, RESET, RESET);
        println!("{}{}{} {}{}{}", CYAN, BOLD, lang.command, command, RESET, RESET);
        println!("{}{}{} {:?}{}{}", CYAN, BOLD, lang.args, command_args, RESET, RESET);
    }
    
    // --- 第一部分：创建 PTY 并准备 AI 命令 ---
    
    let (pty_master_reader, pts) = open()?;

    println!("{} {}...", lang.preparing, command);
    
    let mut ai_child = PtyCommand::new(&command)
        .args(&command_args)
        .spawn(pts)?;
    
    println!("{}{}{} {}{}{}", CYAN, BOLD, command, lang.started, RESET, RESET);
    println!("{}", "═".repeat(60));

    // --- 流式 Markdown 渲染 ---
    let mut reader = BufReader::new(pty_master_reader);
    let mut line = String::new();
    let mut renderer = MarkdownRenderer::new();

    loop {
        line.clear();
        if debug_mode {
            println!("{}{}{}{}{}", CYAN, BOLD, lang.reading_next_line, RESET, RESET);
        }
        let bytes_read = reader.read_line(&mut line)?;
        if debug_mode {
            println!("{}{}{} {}: {:?}{}{}", CYAN, BOLD, lang.bytes_read.replace("{}", &bytes_read.to_string()), bytes_read, line.trim(), RESET, RESET);
        }

        if bytes_read == 0 {
            if debug_mode {
                println!("{}{}{}{}{}", CYAN, BOLD, lang.read_finished, RESET, RESET);
            }
            break;
        }

        // 在debug模式下显示原始内容
        if debug_mode {
            print!("{}{}[raw]{} {}", YELLOW, BOLD, RESET, line);
            std::io::stdout().flush()?;
        }

        // 立即渲染并输出当前行
        if debug_mode {
            println!("{}{}{}{}{}", CYAN, BOLD, lang.starting_render, RESET, RESET);
        }
        let rendered = renderer.render_line(&line);
        if debug_mode {
            println!("{}{}{} {}{}{}", CYAN, BOLD, lang.render_completed, rendered.len(), RESET, RESET);
        }
        print!("{}", rendered);
        std::io::stdout().flush()?;
    }

    println!("\n{}", "═".repeat(60));
    println!("{}{}{}{}{}", GREEN, BOLD, lang.completed, RESET, RESET);

    // --- 等待 AI 进程结束 ---
    let ai_status = ai_child.wait()?;
    println!("{} {}: {:?}", command, lang.exited, ai_status);

    Ok(())
}
