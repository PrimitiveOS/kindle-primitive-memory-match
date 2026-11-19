use std::{
    fs, io,
    path::PathBuf,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use memorypairs_core::{Board, Card, Game};
use primitive_app::{App, Change, HEIGHT, WIDTH};
use primitive_ui::{
    back_button, header, screen_rect, Canvas, Icon, Input, Rect, Sheet, SheetAction, BLACK, BODY,
    CAPTION, PAPER, SECONDARY,
};

const MISMATCH_DELAY: Duration = Duration::from_secs(1);
const BACK: Icon = Icon {
    stroke: &["M12 3 L21 12 L12 21 L3 12 Z", "M12 8 L16 12 L12 16 L8 12 Z"],
    fill: &[],
};
const SYMBOLS: [Icon; 10] = [
    Icon { stroke: &["M12 7 A5 5 0 1 0 12 17 A5 5 0 1 0 12 7", "M12 2 V4 M12 20 V22 M2 12 H4 M20 12 H22 M5 5 L7 7 M17 17 L19 19 M5 19 L7 17 M17 7 L19 5"], fill: &[] },
    Icon { stroke: &[], fill: &["M14 2 C6 2 2 7 2 12 C2 18 7 22 12 22 C17 22 20 19 22 15 C16 18 10 14 10 9 C10 6 12 3 14 2 Z"] },
    Icon { stroke: &[], fill: &["M12 2 L15 9 L23 9 L17 14 L19 22 L12 17 L5 22 L7 14 L1 9 L9 9 Z"] },
    Icon { stroke: &["M5 16 C1 10 8 4 20 3 C20 14 16 20 10 20 C6 20 4 18 5 16 Z", "M3 22 L16 8"], fill: &[] },
    Icon { stroke: &["M2 7 C5 4 8 4 12 7 C16 10 19 10 22 7", "M2 12 C5 9 8 9 12 12 C16 15 19 15 22 12", "M2 17 C5 14 8 14 12 17 C16 20 19 20 22 17"], fill: &[] },
    Icon { stroke: &["M9 3 A5 5 0 1 0 9 13 A5 5 0 1 0 9 3", "M13 12 L21 20 L18 23 M16 15 L19 12"], fill: &[] },
    Icon { stroke: &["M12 3 L4 16 H20 Z", "M12 16 V22 M8 22 H16"], fill: &[] },
    Icon { stroke: &[], fill: &["M13 2 L4 14 H11 L9 22 L20 10 H13 L15 2 Z"] },
    Icon { stroke: &["M2 20 L9 6 L13 13 L16 9 L22 20 Z", "M7 11 L9 13 L11 11"], fill: &[] },
    Icon { stroke: &["M12 2 A3 3 0 1 0 12 8 A3 3 0 1 0 12 2", "M12 8 V21 M8 12 H16", "M3 14 C3 20 7 22 12 22 C17 22 21 20 21 14 M2 16 L3 14 L5 16 M19 16 L21 14 L22 16"], fill: &[] },
];

pub struct View {
    pub game: Game,
    path: Option<PathBuf>,
    pending_board: Option<Board>,
    hide_mismatch_at: Option<Instant>,
    notice: Option<&'static str>,
}

impl View {
    pub fn open(path: PathBuf) -> Self {
        let mut game = fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .map(Game::restore)
            .unwrap_or_else(|| Game::new(Board::Medium, random_seed()));
        game.hide_mismatch();
        Self {
            game,
            path: Some(path),
            pending_board: None,
            hide_mismatch_at: None,
            notice: None,
        }
    }

    pub fn preview(board: Board, seed: u64) -> Self {
        Self {
            game: Game::new(board, seed),
            path: None,
            pending_board: None,
            hide_mismatch_at: None,
            notice: None,
        }
    }

    fn changed(&mut self, change: Change) -> Change {
        self.notice = if self.save().is_ok() {
            None
        } else {
            Some("Could not save this game.")
        };
        change
    }

    fn start(&mut self, board: Board) -> Change {
        if self.game.started() && !self.game.complete() {
            self.pending_board = Some(board);
            return Change::Full;
        }
        self.restart(board)
    }

    fn restart(&mut self, board: Board) -> Change {
        self.hide_mismatch_at = None;
        self.game.restart(board, random_seed());
        self.changed(Change::Full)
    }

    fn tap(&mut self, x: f32, y: f32) -> Change {
        let width = WIDTH as i32;
        if let Some(board) = self.pending_board {
            let Some(action) = new_game_sheet(Rect::new(0, 0, width, HEIGHT as i32)).hit(x, y)
            else {
                return Change::None;
            };
            self.pending_board = None;
            if action == SheetAction::Confirm {
                return self.restart(board);
            }
            return Change::Full;
        }

        if back_button(width).contains(x, y) {
            return Change::Exit;
        }
        if new_button(width).contains(x, y) {
            return self.start(self.game.board());
        }
        let board = self.game.board();
        for (index, size) in Board::ALL.into_iter().enumerate() {
            if size != board && mode_button(width, index).contains(x, y) {
                return self.start(size);
            }
        }

        let card = (0..board.cards()).find(|&index| card_rect(width, board, index).contains(x, y));
        if card.is_some_and(|index| self.game.turn(index)) {
            let change = self.changed(Change::Partial);
            if self.game.mismatch() {
                self.hide_mismatch_at = Some(Instant::now() + MISMATCH_DELAY);
            }
            return change;
        }
        Change::None
    }

    fn draw_cards(&self, canvas: &mut Canvas, width: i32) {
        let board = self.game.board();
        for index in 0..board.cards() {
            let rect = card_rect(width, board, index);
            let card = self.game.card(index);
            let pressed = card == Card::Hidden
                && self.pending_board.is_none()
                && !self.game.mismatch()
                && canvas.control_pressed(rect, false);
            let (symbol, filled, border) = match card {
                Card::Hidden => (BACK, pressed, 3),
                Card::Face(symbol) => (SYMBOLS[usize::from(symbol)], false, 7),
                Card::Matched(symbol) => (SYMBOLS[usize::from(symbol)], true, 3),
            };
            canvas.panel(rect, 18, border, if filled { BLACK } else { PAPER });
            let length = if card == Card::Hidden {
                canvas.px(63)
            } else {
                canvas.px(110)
            };
            let glyph = Rect::new(
                rect.x + (rect.w - length) / 2,
                rect.y + (rect.h - length) / 2,
                length,
                length,
            );
            let ink = if filled {
                PAPER
            } else if card == Card::Hidden {
                SECONDARY
            } else {
                BLACK
            };
            canvas.icon(glyph, symbol, ink);
        }
    }
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let width = canvas.bounds().w;
        header(canvas, "Memory Match", "Done");
        let board = self.game.board();
        canvas.card(screen_rect(width, 40, 165, 1184, 111));
        canvas.text(
            screen_rect(width, 67, 181, 550, 51),
            &format!("{} of {} pairs", self.game.pairs(), board.cards() / 2),
            BODY.bold(),
        );
        let attempts = self.game.attempts();
        let attempt_word = if attempts == 1 { "attempt" } else { "attempts" };
        canvas.text(
            screen_rect(width, 67, 231, 550, 39),
            &format!("{attempts} {attempt_word}"),
            CAPTION,
        );
        let record = self
            .game
            .best()
            .map_or_else(|| "—".to_string(), |best| best.to_string());
        canvas.text(
            screen_rect(width, 678, 181, 519, 51),
            &record,
            BODY.bold().right(),
        );
        canvas.text(
            screen_rect(width, 678, 231, 519, 39),
            "Best attempts",
            CAPTION.right(),
        );
        let instruction = if self.game.complete() {
            "All pairs found."
        } else if self.game.mismatch() {
            "No match."
        } else if self.game.one_revealed() {
            "Turn over a second card."
        } else {
            "Turn over two cards to find a match."
        };
        canvas.text(
            screen_rect(width, 40, 289, 1184, 54),
            instruction,
            BODY.center(),
        );
        self.draw_cards(canvas, width);
        canvas.text(
            screen_rect(width, 40, 1293, 1184, 49),
            "Board size",
            CAPTION,
        );
        for (index, size) in Board::ALL.into_iter().enumerate() {
            canvas.button(
                mode_button(width, index),
                &format!("{} cards", size.cards()),
                size == board,
                false,
            );
        }
        canvas.button(new_button(width), "New game", self.game.complete(), false);
        if let Some(notice) = self.notice {
            canvas.text(
                screen_rect(width, 40, 1602, 1184, 50),
                notice,
                CAPTION.center(),
            );
        }
        if self.pending_board.is_some() {
            new_game_sheet(canvas.bounds()).draw(canvas, None);
        }
    }

    fn input(&mut self, input: Input) -> Change {
        match input {
            Input::Tap(x, y) => self.tap(x, y),
            Input::PageBack if self.pending_board.is_some() => {
                self.pending_board = None;
                Change::Full
            }
            Input::PageBack => Change::Exit,
            Input::Wake => self.tick(Instant::now()),
            _ => Change::None,
        }
    }

    fn deadline(&self) -> Option<Instant> {
        self.hide_mismatch_at
    }

    fn tick(&mut self, now: Instant) -> Change {
        if self
            .hide_mismatch_at
            .is_some_and(|deadline| now >= deadline)
        {
            self.hide_mismatch_at = None;
            if self.game.hide_mismatch() {
                return self.changed(Change::Partial);
            }
        }
        Change::None
    }

    fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.path {
            primitive_app::save(path, &serde_json::to_vec(self.game.snapshot())?)?;
        }
        Ok(())
    }
}

fn card_rect(width: i32, board: Board, index: usize) -> Rect {
    let cols = board.columns() as i32;
    let col = index as i32 % cols;
    let row = index as i32 / cols;
    let gap = 16;
    let card_width = (1184 - (cols - 1) * gap) / cols;
    let top = if board == Board::Small { 418 } else { 352 };
    screen_rect(
        width,
        40 + col * (card_width + gap),
        top + row * (216 + gap),
        card_width,
        216,
    )
}

fn mode_button(width: i32, index: usize) -> Rect {
    screen_rect(width, 40 + index as i32 * 400, 1344, 384, 103)
}

fn new_button(width: i32) -> Rect {
    screen_rect(width, 400, 1480, 464, 103)
}

fn new_game_sheet(bounds: Rect) -> Sheet<'static> {
    Sheet::centered(
        bounds,
        "Start a new game?",
        "Your current game will end. Your best scores stay saved.",
        "New game",
    )
}

fn random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

pub fn draw_icon(canvas: &mut Canvas) {
    canvas.begin();
    canvas.panel(Rect::new(29, 53, 125, 163), 61, 24, PAPER);
    canvas.icon(Rect::new(43, 77, 91, 91), BACK, BLACK);
    canvas.panel(Rect::new(96, 26, 128, 165), 61, 23, BLACK);
    canvas.icon(Rect::new(112, 62, 97, 97), SYMBOLS[2], PAPER);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tap_card(app: &mut View, index: usize) -> Change {
        let card = card_rect(WIDTH as i32, app.game.board(), index);
        app.input(Input::Tap(
            (card.x + card.w / 2) as f32,
            (card.y + card.h / 2) as f32,
        ))
    }

    #[test]
    fn mismatch_has_one_deadline_and_blocks_card_touches_until_it_expires() {
        let mut app = View::preview(Board::Medium, 2026);
        assert_eq!(tap_card(&mut app, 2), Change::Partial);
        assert_eq!(app.deadline(), None);
        assert_eq!(tap_card(&mut app, 3), Change::Partial);
        assert!(app.game.mismatch());
        let deadline = app.deadline().unwrap();
        assert_eq!(tap_card(&mut app, 1), Change::None);
        assert_eq!(app.tick(deadline - Duration::from_millis(1)), Change::None);
        assert_eq!(app.tick(deadline), Change::Partial);
        assert_eq!(app.game.card(2), Card::Hidden);
        assert_eq!(app.game.card(3), Card::Hidden);
        assert_eq!(app.game.attempts(), 1);
        assert_eq!(app.deadline(), None);
        assert_eq!(app.tick(deadline), Change::None);
        assert_eq!(tap_card(&mut app, 1), Change::Partial);
    }

    #[test]
    fn reopened_mismatch_is_hidden_and_preserves_the_attempt() {
        let mut app = View::preview(Board::Medium, 2026);
        assert_eq!(tap_card(&mut app, 2), Change::Partial);
        assert_eq!(tap_card(&mut app, 3), Change::Partial);
        let path = std::env::temp_dir().join(format!(
            "primitive-memorypairs-reopen-{}-{}.json",
            std::process::id(),
            random_seed()
        ));
        fs::write(&path, serde_json::to_vec(app.game.snapshot()).unwrap()).unwrap();
        let mut reopened = View::open(path.clone());
        fs::remove_file(path).unwrap();
        reopened.path = None;
        assert_eq!(reopened.deadline(), None);
        assert_eq!(reopened.game.card(2), Card::Hidden);
        assert_eq!(reopened.game.card(3), Card::Hidden);
        assert_eq!(reopened.game.attempts(), 1);
        assert_eq!(tap_card(&mut reopened, 1), Change::Partial);
    }
}
