use ratatui::{
    backend::TestBackend, // 在 WASM 中我們通常先用 TestBackend 或自定義 Backend 獲取數據流
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;

fn main() -> Result<(), io::Error> {
    // 初始化一個虛擬的 Backend (稍後會適配 WASM 渲染)
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    // 第一個渲染循環
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(f.size());

        // 標題區域
        let header = Block::default()
            .borders(Borders::ALL)
            .title("$PRSDKE_ARCHITECT_LOG")
            .style(Style::default().fg(Color::Green).bg(Color::Black));
        
        let header_text = Paragraph::new(" > [STATUS: ONLINE] | SYSTEM: RUST_WASM_TUI | BRAND: $PRSDKE")
            .block(header);
        f.render_widget(header_text, chunks[0]);

        // 主交互區域 (RAG 回答區佔位)
        let body = Block::default()
            .borders(Borders::ALL)
            .title(" COMMAND_LINE_INTERFACE ")
            .style(Style::default().fg(Color::Green).bg(Color::Black));
        
        let welcome_msg = ""
        let content = Paragraph::new(welcome_msg)
            .block(body)
            .wrap(Wrap { trim: true });
        f.render_widget(content, chunks[1]);
    })?;

    Ok(())
}