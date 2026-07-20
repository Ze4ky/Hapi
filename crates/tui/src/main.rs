use std::{fs::read_to_string, io};

use hapi_core::{Api, model::RequestInfo};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier},
    text::{Line, Span},
    widgets::{Block, List, ListState, Paragraph, Wrap},
};

fn main() -> std::io::Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let hapi = Api::new();
    ratatui::run(|terminal| app(terminal, &hapi, &rt))?;
    Ok(())
}

fn app(
    terminal: &mut DefaultTerminal,
    hapi: &Api,
    rt: &tokio::runtime::Runtime,
) -> std::io::Result<()> {
    let mut items: Vec<String> = Vec::new();
    let file = match read_to_string("./request.json") {
        Ok(f) => f,
        Err(e) => {
            println!("文件request.json发生错误: {}", e);
            return Err(e);
        }
    };
    let request_info_vec: Vec<RequestInfo> = match serde_json::from_str(&file) {
        Ok(vec) => vec,
        Err(e) => {
            println!("反序列化request.json发生错误: {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidData, e));
        }
    };

    let mut index: i32 = 0;
    let mut info_index: i32 = 0;

    for info in request_info_vec.iter() {
        info_index += 1;
        let title = format!("{}. {}", info_index, info.name.to_string());
        items.insert(index.clone() as usize, title);
        index += 1;
    }

    info_index = 0;

    let mut list_state = ListState::default().with_selected(Some(info_index as usize));

    let mut response_text = String::new();

    loop {
        terminal.draw(|frame| render(frame, &mut list_state, items.clone(), &response_text))?;
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('s') | KeyCode::Down => {
                    list_state.select_next();
                    info_index = list_state.selected().unwrap_or(0) as i32;
                }
                KeyCode::Char('w') | KeyCode::Up => {
                    list_state.select_previous();
                    info_index = list_state.selected().unwrap_or(0) as i32;
                }
                KeyCode::Esc => break Ok(()),
                KeyCode::Enter => {
                    let response = match rt.block_on(
                        hapi.request(index_to_value(info_index, request_info_vec.clone()).clone()),
                    ) {
                        Ok(resp) => serde_json::to_string_pretty(&resp).unwrap(),
                        Err(e) => format!("请求错误: {}", e),
                    };
                    response_text = response;
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, list_state: &mut ListState, items: Vec<String>, response_text: &str) {
    let vertical_constraint = [Constraint::Length(1), Constraint::Fill(1)];
    let horizontal_constraint = [Constraint::Percentage(30), Constraint::Percentage(70)];

    let vertical = Layout::vertical(vertical_constraint).spacing(1);
    let horizontal = Layout::horizontal(horizontal_constraint).spacing(1);

    let [top, main] = frame.area().layout(&vertical);
    let [left, right] = main.layout(&horizontal);
    let title = Line::from_iter([Span::from("Hapi TUI")]);

    frame.render_widget(title.centered(), top);
    render_list(frame, left, list_state, items);
    render_response(frame, right, response_text);
}

fn render_list(frame: &mut Frame, area: Rect, list_state: &mut ListState, items: Vec<String>) {
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::bordered().title("请求列表"));
    frame.render_stateful_widget(list, area, list_state);
}

fn render_response(frame: &mut Frame, area: Rect, response_text: &str) {
    let paragraph = Paragraph::new(response_text)
        .block(Block::bordered().title("响应"))
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn index_to_value(index: i32, request_info_vec: Vec<RequestInfo>) -> RequestInfo {
    request_info_vec[index as usize].clone()
}
