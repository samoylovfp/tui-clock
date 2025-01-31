use std::{f64::consts::TAU, time::Duration};

use chrono::{NaiveTime, Timelike};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Circle, Line},
    Frame,
};

#[derive(Clone, Copy)]
struct Theme {
    bg: Color,
    circle: Color,
    marks: Color,
    hour: Color,
    minute: Color,
    second: Color,
}

fn main() {
    let aspect_ratio: f64 = std::env::args()
        .nth(1)
        .expect("Pass aspect ratio as the first argument, probably around 0.6")
        .parse()
        .expect("pass a float as the first parameter");

    let theme = std::env::args().nth(2).unwrap_or_default();

    let default_theme = Theme {
        bg: Color::Reset,
        circle: Color::Cyan,
        marks: Color::Gray,
        hour: Color::LightRed,
        minute: Color::LightGreen,
        second: Color::White,
    };
    let rose_pine = Theme {
        bg: Color::Rgb(31, 29, 46),
        circle: Color::Rgb(49, 116, 143),
        marks: Color::Rgb(144, 140, 170),
        hour: Color::Rgb(246, 193, 119),
        minute: Color::Rgb(235, 188, 186),
        second: Color::Rgb(156, 207, 216),
    };
    let theme = match theme.as_str() {
        "rose_pine" => rose_pine,
        _ => default_theme,
    };

    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|f| draw(f, aspect_ratio, theme))
            .expect("failed to draw frame");
        if matches!(
            event::poll(Duration::from_millis(10)).expect("failed to read event"),
            true
        ) {
            if matches!(
                event::read().expect("failed to read event"),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                })
            ) {
                break;
            }
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame, aspect_ratio: f64, theme: Theme) {
    let now = chrono::Local::now().time();
    let r: f64 = 1.0;
    let margin = 1.0;
    let braille_marks = true;

    let min_side =
        ((frame.area().width as f64) * aspect_ratio).min(frame.area().height.into()) - margin;
    let width = frame.area().width as f64 / min_side * aspect_ratio;
    let height = frame.area().height as f64 / min_side;

    let clock = Canvas::default()
        .background_color(theme.bg)
        .x_bounds([-width, width])
        .y_bounds([-height, height])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: r,
                color: theme.circle,
            });
            let txt_r = r * 0.90;
            if braille_marks {
                ctx.print(0.0, txt_r, "⡇⣝");
                ctx.print(txt_r, 0.0, "⣽");
                ctx.print(0.0, -txt_r, "⣯");
                ctx.print(-txt_r, 0.0, "⣻");
            } else {
                ctx.print(0.0, txt_r, "12");
                ctx.print(txt_r, 0.0, "3");
                ctx.print(0.0, -txt_r, "6");
                ctx.print(-txt_r, 0.0, "9");
            }
            let second_angle =
                |t: NaiveTime| (t.second() as f64 + t.nanosecond() as f64 / 1e9) / 60.0 * TAU;
            let minute_angle =
                |t: NaiveTime| t.minute() as f64 / 60.0 * TAU + second_angle(t) / 60.0;
            let hour_angle =
                |t: NaiveTime| t.hour12().1 as f64 / 12.0 * TAU + minute_angle(t) / 12.0;
            if !braille_marks {
                for h in [0, 3, 6, 9] {
                    let angle = hour_angle(NaiveTime::from_hms_opt(h, 0, 0).expect("valid time"));
                    let x = angle.sin();
                    let y = angle.cos();
                    let start = 1.0;
                    let end = 0.8;
                    ctx.draw(&Line {
                        x1: x * start,
                        y1: y * start,
                        x2: x * end,
                        y2: y * end,
                        color: theme.marks,
                    });
                }
            }
            for (angle, length, color) in [
                (second_angle(now), 0.8, theme.second),
                (minute_angle(now), 0.7, theme.minute),
                (hour_angle(now), 0.6, theme.hour),
            ] {
                let angle_from_top = angle;
                let x = angle_from_top.sin();
                let y = angle_from_top.cos();
                ctx.draw(&Line {
                    x1: 0.0,
                    y1: 0.0,
                    x2: x * length,
                    y2: y * length,
                    color,
                });
            }
        });
    frame.render_widget(clock, frame.area());
}
