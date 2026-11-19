//! A deterministic deck, turn rules, and saved records for Memory Match.

use serde::{Deserialize, Serialize};

const MAX_CARDS: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Board {
    Small,
    Medium,
    Large,
}

impl Board {
    pub const ALL: [Self; 3] = [Self::Small, Self::Medium, Self::Large];

    pub fn columns(self) -> usize {
        match self {
            Self::Small | Self::Medium => 4,
            Self::Large => 5,
        }
    }

    pub fn rows(self) -> usize {
        match self {
            Self::Small => 3,
            Self::Medium | Self::Large => 4,
        }
    }

    pub fn cards(self) -> usize {
        self.columns() * self.rows()
    }

    fn index(self) -> usize {
        match self {
            Self::Small => 0,
            Self::Medium => 1,
            Self::Large => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Revealed {
    None,
    One(usize),
    Mismatch(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Hidden,
    Face(u8),
    Matched(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    board: Board,
    seed: u64,
    matched: u32,
    revealed: Revealed,
    attempts: u32,
    best: [Option<u32>; 3],
}

pub struct Game {
    state: Snapshot,
    deck: [u8; MAX_CARDS],
}

impl Game {
    pub fn new(board: Board, seed: u64) -> Self {
        Self {
            deck: shuffle(board, seed),
            state: Snapshot {
                board,
                seed,
                matched: 0,
                revealed: Revealed::None,
                attempts: 0,
                best: [None; 3],
            },
        }
    }

    pub fn restore(snapshot: Snapshot) -> Self {
        let mut game = Self::new(snapshot.board, snapshot.seed);
        game.state.best = snapshot.best;
        for board in Board::ALL {
            let record = &mut game.state.best[board.index()];
            *record = record.filter(|best| *best >= board.cards() as u32 / 2);
        }
        if game.valid_round(&snapshot) {
            game.state.matched = snapshot.matched;
            game.state.revealed = snapshot.revealed;
            game.state.attempts = snapshot.attempts;
            game.record_if_complete();
        }
        game
    }

    fn valid_round(&self, snapshot: &Snapshot) -> bool {
        let count = self.board().cards();
        if snapshot.matched >> count != 0 || snapshot.matched.count_ones() / 2 > snapshot.attempts {
            return false;
        }
        let mut matches = [0; MAX_CARDS / 2];
        for (index, &symbol) in self.deck[..count].iter().enumerate() {
            if snapshot.matched & (1 << index) != 0 {
                matches[usize::from(symbol)] += 1;
            }
        }
        if matches.contains(&1) {
            return false;
        }
        let open = |index: usize| index < count && snapshot.matched & (1 << index) == 0;
        match snapshot.revealed {
            Revealed::None => true,
            Revealed::One(index) => open(index),
            Revealed::Mismatch(a, b) => {
                snapshot.attempts > snapshot.matched.count_ones() / 2
                    && open(a)
                    && open(b)
                    && self.deck[a] != self.deck[b]
            }
        }
    }

    pub fn snapshot(&self) -> &Snapshot {
        &self.state
    }

    pub fn board(&self) -> Board {
        self.state.board
    }

    pub fn attempts(&self) -> u32 {
        self.state.attempts
    }

    pub fn pairs(&self) -> u32 {
        self.state.matched.count_ones() / 2
    }

    pub fn best(&self) -> Option<u32> {
        self.state.best[self.board().index()]
    }

    pub fn complete(&self) -> bool {
        self.pairs() as usize * 2 == self.board().cards()
    }

    pub fn started(&self) -> bool {
        self.state.attempts > 0 || self.state.revealed != Revealed::None
    }

    pub fn mismatch(&self) -> bool {
        matches!(self.state.revealed, Revealed::Mismatch(_, _))
    }

    pub fn one_revealed(&self) -> bool {
        matches!(self.state.revealed, Revealed::One(_))
    }

    pub fn card(&self, index: usize) -> Card {
        if self.is_matched(index) {
            return Card::Matched(self.deck[index]);
        }
        match self.state.revealed {
            Revealed::One(first) if first == index => Card::Face(self.deck[index]),
            Revealed::Mismatch(first, second) if first == index || second == index => {
                Card::Face(self.deck[index])
            }
            _ => Card::Hidden,
        }
    }

    pub fn turn(&mut self, index: usize) -> bool {
        if index >= self.board().cards() || self.is_matched(index) {
            return false;
        }
        self.state.revealed = match self.state.revealed {
            Revealed::One(first) if first == index => return false,
            Revealed::One(first) => {
                self.state.attempts = self.state.attempts.saturating_add(1);
                if self.deck[first] == self.deck[index] {
                    self.state.matched |= (1 << first) | (1 << index);
                    self.record_if_complete();
                    Revealed::None
                } else {
                    Revealed::Mismatch(first, index)
                }
            }
            Revealed::Mismatch(_, _) => return false,
            Revealed::None => Revealed::One(index),
        };
        true
    }

    pub fn hide_mismatch(&mut self) -> bool {
        if !self.mismatch() {
            return false;
        }
        self.state.revealed = Revealed::None;
        true
    }

    fn is_matched(&self, index: usize) -> bool {
        index < self.board().cards() && self.state.matched & (1 << index) != 0
    }

    fn record_if_complete(&mut self) {
        if self.complete() {
            let previous = self.best();
            let attempts = self.state.attempts;
            self.state.best[self.board().index()] =
                Some(previous.map_or(attempts, |best| best.min(attempts)));
        }
    }

    pub fn restart(&mut self, board: Board, seed: u64) {
        let best = self.state.best;
        *self = Self::new(board, seed);
        self.state.best = best;
    }
}

fn shuffle(board: Board, mut seed: u64) -> [u8; MAX_CARDS] {
    let mut deck = std::array::from_fn(|index| (index / 2) as u8);
    for i in (1..board.cards()).rev() {
        // SplitMix64 supplies a stable shuffle across 32-bit and 64-bit builds.
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut value = seed;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
        value ^= value >> 31;
        deck.swap(i, (value % (i as u64 + 1)) as usize);
    }
    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete(mut game: Game) -> Game {
        for symbol in 0..game.board().cards() as u8 / 2 {
            let cells: Vec<_> = game.deck[..game.board().cards()]
                .iter()
                .enumerate()
                .filter_map(|(index, &card)| (card == symbol).then_some(index))
                .collect();
            assert_eq!(cells.len(), 2);
            assert!(game.turn(cells[0]) && game.turn(cells[1]));
            assert_eq!(game.card(cells[0]), Card::Matched(symbol));
        }
        game
    }

    #[test]
    fn all_board_sizes_can_be_completed_and_keep_separate_records() {
        let mut game = Game::new(Board::Small, 2026);
        for board in Board::ALL {
            game.restart(board, 2026);
            game = complete(game);
            assert!(game.complete());
            assert_eq!(game.best(), Some(board.cards() as u32 / 2));
        }
        game.restart(Board::Small, 8361);
        assert_eq!(game.best(), Some(6));
        assert_eq!(game.pairs(), 0);
    }

    #[test]
    fn mismatch_blocks_turns_until_both_cards_are_hidden() {
        let mut game = Game::new(Board::Medium, 4);
        let other = (1..game.board().cards())
            .find(|&index| game.deck[index] != game.deck[0])
            .unwrap();
        let next = (1..game.board().cards())
            .find(|&index| index != other)
            .unwrap();
        assert!(game.turn(0));
        assert!(!game.turn(0));
        assert!(game.turn(other));
        assert!(game.mismatch());
        assert!(!game.turn(other));
        assert!(!game.turn(next));
        assert!(game.hide_mismatch());
        assert!(!game.hide_mismatch());
        assert_eq!(game.card(0), Card::Hidden);
        assert_eq!(game.card(other), Card::Hidden);
        assert!(game.turn(next));
        assert_eq!(game.attempts(), 1);
    }

    #[test]
    fn valid_mismatch_restores_and_bad_progress_keeps_records() {
        let mut game = Game::new(Board::Medium, 82);
        let other = (1..game.board().cards())
            .find(|&index| game.deck[index] != game.deck[0])
            .unwrap();
        game.turn(0);
        game.turn(other);
        let saved = game.snapshot().clone();
        let game = Game::restore(saved.clone());
        assert!(game.mismatch());
        assert_eq!(game.attempts(), 1);
        assert_eq!(game.card(0), Card::Face(game.deck[0]));
        let mut broken = saved;
        broken.matched = u32::MAX;
        broken.best[1] = Some(9);
        let restored = Game::restore(broken);
        assert_eq!(restored.attempts(), 0);
        assert_eq!(restored.pairs(), 0);
        assert_eq!(restored.best(), Some(9));
    }
}
