use std::{
    f64::consts::{FRAC_PI_2, PI, TAU},
    time::Duration,
};

use chrono::Timelike;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    style::Color,
    text::Text,
    widgets::canvas::{Canvas, Circle, Line},
    Frame,
};

fn main() {
    let aspect_ratio: f64 = std::env::args()
        .nth(1)
        .expect("Pass aspect ratio as an argument, probably around 0.6")
        .parse()
        .expect("pass a float as a first parameter");
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|f| draw(f, aspect_ratio))
            .expect("failed to draw frame");
        if matches!(
            event::poll(Duration::from_millis(10)).expect("failed to read event"),
            true
        ) {
            if matches!(
                event::read().expect("event"),
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

fn draw(frame: &mut Frame, aspect_ratio: f64) {
    let now = chrono::Local::now();
    let r: f64 = 0.95;
    let margin = 1.0;

    let min_side =
        ((frame.area().width as f64) * aspect_ratio).min(frame.area().height.into()) - margin;
    let width = frame.area().width as f64 / min_side * aspect_ratio;
    let height = frame.area().height as f64 / min_side;

    let clock = Canvas::default()
        .x_bounds([-width, width])
        .y_bounds([-height, height])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: r,
                color: Color::Cyan,
            });
            let txt_r = r * 0.90;
            ctx.print(0.0, txt_r, "12");
            ctx.print(txt_r, 0.0, "3");
            ctx.print(0.0, -txt_r, "6");
            ctx.print(-txt_r, 0.0, "9");
            let second_angle =
                (now.second() as f64 + now.timestamp_subsec_millis() as f64 / 1000.0) / 60.0 * TAU;
            let minute_angle = now.minute() as f64 / 60.0 * TAU + second_angle / 60.0;
            let hour_angle = now.hour12().1 as f64 / 12.0 * TAU + minute_angle / 60.0;

            for (length, angle, color) in [
                (0.8, second_angle, Color::White),
                (0.7, minute_angle, Color::LightGreen),
                (0.6, hour_angle, Color::LightRed),
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
