mod app;
mod gopher;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders,
    },
    Terminal,
};
use std::rc::Rc;
use std::{
    io,
    time::{Duration, Instant},
};

use app::App;
use gopher::gopher;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    Ok(loop {
        terminal.draw(|frame| {
            let item = Canvas::default()
                .block(
                    Block::default()
                        .title("ダンシング・ゴーファー")
                        .borders(Borders::ALL),
                )
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.layer();
                    gopher().into_iter().for_each(|r| {
                        r.into_iter().for_each(|(x, width, y, color)| {
                            ctx.draw(&Rectangle {
                                x: x + app.x,
                                y: y + app.y,
                                width,
                                height: 1.0,
                                color,
                            });
                        })
                    });
                });
            frame.render_widget(item, frame.size());
        })?;
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
                if KeyCode::Left == key.code {
                    app.left();
                }
                if KeyCode::Right == key.code {
                    app.right();
                }
                if KeyCode::Up == key.code {
                    app.jump();
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.moving();
            last_tick = Instant::now();
        }
    })
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let app = App::new(0.0, 0.0);
    let tick_rate = Duration::from_millis(50);
    run(&mut terminal, app, tick_rate)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
