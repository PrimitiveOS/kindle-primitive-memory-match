use std::{
    fs,
    path::{Path, PathBuf},
};

use memorypairs_core::Board;
use memorypairs_render::{draw_icon, View};
use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let paths = font_paths(&fonts);
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &paths)?;
    let mut icon = Canvas::new(256, 256, &paths)?;
    draw_icon(&mut icon);
    write_png(&output.join("icon.png"), 256, 256, &icon.finish(false))?;

    let mut app = View::preview(Board::Medium, 2026);
    assert_eq!(app.input(Input::Tap(182.0, 452.0)), Change::Partial);
    assert_eq!(app.input(Input::Tap(782.0, 685.0)), Change::Partial);
    assert_eq!(app.game.pairs(), 1);
    assert_eq!(app.input(Input::Tap(782.0, 454.0)), Change::Partial);
    write_png(
        &output.join("memorypairs.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    assert_eq!(app.input(Input::Tap(1082.0, 454.0)), Change::Partial);
    assert!(app.game.mismatch());
    let deadline = app.deadline().expect("a mismatch schedules one redraw");
    assert_eq!(app.input(Input::Tap(482.0, 454.0)), Change::None);
    assert_eq!(
        app.tick(deadline - std::time::Duration::from_millis(1)),
        Change::None
    );
    assert!(app.game.mismatch());
    assert_eq!(app.tick(deadline), Change::Partial);
    assert!(!app.game.mismatch());
    assert_eq!(app.deadline(), None);
    assert_eq!(app.tick(deadline), Change::None);
    assert_eq!(app.input(Input::Tap(482.0, 454.0)), Change::Partial);
    assert!(app.game.one_revealed());

    let mut large = View::preview(Board::Large, 2026);
    for (x, y) in [
        (151.0, 452.0),
        (1111.0, 685.0),
        (631.0, 450.0),
        (151.0, 685.0),
        (867.0, 450.0),
    ] {
        assert_eq!(large.input(Input::Tap(x, y)), Change::Partial);
    }
    assert_eq!(large.game.pairs(), 2);
    write_png(
        &output.join("memorypairs-playing.png"),
        WIDTH,
        HEIGHT,
        &frame(&large, &mut canvas, true),
    )?;
    assert_eq!(large.input(Input::Tap(238.0, 1390.0)), Change::Full);
    assert_eq!(large.input(Input::PageBack), Change::Full);
    assert_eq!(large.game.board(), Board::Large);
    assert_eq!(large.input(Input::Tap(238.0, 1390.0)), Change::Full);
    assert_eq!(large.input(Input::Tap(846.0, 1000.0)), Change::Full);
    assert_eq!(large.game.board(), Board::Small);
    assert_eq!(large.game.attempts(), 0);
    assert_eq!(large.input(Input::Tap(1110.0, 85.0)), Change::Exit);
    Ok(())
}
