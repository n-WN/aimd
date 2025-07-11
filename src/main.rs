use std::error::Error;
use std::io::{Write, BufRead, BufReader, IsTerminal, Read};
use pty_process::blocking::{open, Command as PtyCommand};

use aimd::{detect_language, MarkdownRenderer, parse_args, CYAN, BOLD, RESET, GREEN, YELLOW};

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
    if lang as *const _ == &aimd::CHINESE as *const _ {
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
    if lang as *const _ == &aimd::CHINESE as *const _ {
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
