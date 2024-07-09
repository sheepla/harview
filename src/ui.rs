use crate::app::{App, HeaderInfo, TabBarState, TableItem};
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &mut App, frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .split(frame.size());

    render_table(app, main_layout[0], frame.buffer_mut());
    render_preview(app, main_layout[1], frame.buffer_mut());
}

pub fn render_table(app: &mut App, area: Rect, buf: &mut Buffer) {
    let table = EntriesTable::init(&app);
    let mut state = TableState::default();
    state.select(Some(app.get_index()));
    table.render(area, buf, &mut state);
}

pub fn render_preview(app: &mut App, area: Rect, buf: &mut Buffer) {
    let preview = PreviewWidget::init(app);
    preview.render(area, buf);
}

#[derive(Debug)]
pub struct EntriesTable {
    table_items: Vec<TableItem>,
}

impl<'a> EntriesTable {
    pub fn init(app: &App) -> Self {
        let mut state = TableState::default();
        let index = app.get_index();
        state.select(Some(index));

        Self {
            table_items: app.har_data.to_table_items(),
        }
    }

    fn table(&self) -> Table {
        let headers = Row::new(vec![
            Cell::from("Status"),
            Cell::from("Method"),
            Cell::from("Domain"),
            Cell::from("FileName"),
            Cell::from("ContentType"),
            Cell::from("       Size"),
        ])
        .style(Style::default().bold().underlined());

        let widths = [
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Length(12),
            Constraint::Length(12),
        ];

        let rows: Vec<Row> = self
            .table_items
            .iter()
            .map(|item| item.to_table_row())
            .collect();

        Table::new(rows, &widths)
            .header(headers)
            .highlight_style(Style::default().reversed())
            .block(
                Block::default()
                    .padding(Padding::horizontal(1))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
    }
}

impl StatefulWidget for EntriesTable {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let table = self.table();

        StatefulWidget::render(table, area, buf, state);
    }
}

#[derive(Debug)]
struct PreviewWidget {
    tabbar_state: TabBarState,
}

impl PreviewWidget {
    pub fn init(app: &App) -> Self {
        Self {
            tabbar_state: app.tabbar_state.clone(),
        }
    }

    fn tabbar(&self) -> Tabs {
        Tabs::new(vec![
            " [1] Headers ",
            " [2] Cookies ",
            " [3] Request ",
            " [4] Response ",
        ])
        .select(self.tabbar_state.to_index())
        .padding(" ", " ")
    }
}

impl Widget for PreviewWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let tabbar = self.tabbar();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(area);

        Widget::render(tabbar, layout[0], buf);
    }
}

#[derive(Debug)]
struct HeaderPreview {
    header_info: Option<HeaderInfo>,
}

impl HeaderPreview {
    pub fn init(app: &App) -> Self {
        Self {
            header_info: app.har_data.to_header_info(app.get_index()),
        }
    }
}

impl Widget for HeaderPreview {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(header_info) = self.header_info {
            todo!();
        }
    }
}
