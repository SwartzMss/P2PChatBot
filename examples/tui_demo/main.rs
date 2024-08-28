use tokio::sync::mpsc;
use tui::{Terminal, widgets::{Widget, Block, Borders, Paragraph}, layout::{Layout, Constraint, Direction}, backend::CrosstermBackend, text::{Spans, Span}};
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use std::io;

async fn draw_ui() -> crossterm::Result<()> {
    // 初始化终端后端
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 启用原始模式并进入替代屏幕
    terminal::enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    loop {
        // 绘制用户界面
        terminal.draw(|f| {
            // 将界面分为三个部分：顶部的基本信息和底部的两个并排区域
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(20),  // 顶部区域
                        Constraint::Percentage(80),  // 底部区域
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // 基本信息区域
            let block1 = Block::default().title("基本信息").borders(Borders::ALL);
            let paragraph1 = Paragraph::new(vec![
                Spans::from(vec![Span::raw("名字：王小明")]),
                Spans::from(vec![Span::raw("职业：软件开发工程师")]),
                Spans::from(vec![Span::raw("兴趣：编程、阅读、运动")]),
            ])
            .block(block1);
            f.render_widget(paragraph1, chunks[0]);

            // 将底部区域再分为左右两个部分
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),  // 左侧区域
                        Constraint::Percentage(50),  // 右侧区域
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            // 打印区域一（左侧）
            let block2 = Block::default().title("打印区域一").borders(Borders::ALL);
            let paragraph2 = Paragraph::new(vec![
                Spans::from(vec![Span::raw("欢迎来到Rust编程的世界！")]),
                Spans::from(vec![Span::raw("今天我们要学习如何使用tui和tokio来创建一个简单的异步用户界面。")]),
                Spans::from(vec![Span::raw("让我们开始吧！")]),
            ])
            .block(block2);
            f.render_widget(paragraph2, bottom_chunks[0]);

            // 打印区域二（右侧）
            let block3 = Block::default().title("打印区域二").borders(Borders::ALL);
            let paragraph3 = Paragraph::new(vec![
                Spans::from(vec![Span::raw("这是第二个打印区域的内容。")]),
                Spans::from(vec![Span::raw("希望你学到了很多有用的知识！")]),
                Spans::from(vec![Span::raw("记住，编程的道路上，持续学习和实践是最重要的。")]),
            ])
            .block(block3);
            f.render_widget(paragraph3, bottom_chunks[1]);
        })?;

        // 读取键盘事件
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,  // 按 'q' 键退出
                _ => {}
            }
        }
    }

    // 退出时清理终端
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() {

    draw_ui().await;
}
