use super::{AppState,Error};
use crate::{
    game::{GameStatus,GameState,Player},
    api::{
        play::Game,
        submissions::Submission,
    },
};
use std::collections::HashMap;
use rand::{seq::SliceRandom,rngs::SmallRng,SeedableRng};
use rocket::{post,serde::json::Json};
use serde::{
    ser::{Serializer,SerializeStruct},
    Serialize
};
use skillratings::{elo::{elo,EloConfig,EloRating},Outcomes};

// The original code was clearly never designed for this

#[derive(Serialize,Debug)]
pub struct Scoreboard {
    scores: HashMap<String,Score>,
}

#[derive(Debug,Copy,Clone,Default)]
pub struct Score {
    elo: EloRating,
    wins: u16,
    losses: u16,
    draws: u16,
}
// For Elo rating
impl Serialize for Score {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut s = serializer.serialize_struct("Score", 4)?;
        s.serialize_field("elo", &self.elo.rating)?;
        s.serialize_field("wins", &self.wins)?;
        s.serialize_field("losses", &self.losses)?;
        s.serialize_field("draws", &self.draws)?;
        s.end()
    }
}

struct AiGame {
    game: Game,
    w_player: Submission,
    b_player: Submission,
    turns: u8,
}

impl AiGame {
    async fn play(&mut self) -> Option<GameResult> {
        let current_player = self.game.checkers.current_player;
        let current_submission =
            if current_player == Player::White { &self.w_player }
            else { &self.b_player };

        let _ = self.game.play_ai(current_submission.clone()).await;

        // Handle draw
        // TODO: Is this needed?
        if self.turns >= 100 {
            print!("Draw: ");
            return Some(GameResult {
                winner: self.w_player.clone(),
                loser: self.b_player.clone(),
                draw:true
            });
        }

        // Handle victory
        if let GameStatus::Victory(winner) = self.game.checkers.status {
            print!("Victory: ");
            let (winner,loser) = if winner == Player::White { (self.w_player.clone(), self.b_player.clone()) }
            else { (self.b_player.clone(), self.w_player.clone()) };
            return Some(GameResult {
                winner,
                loser,
                draw: false
            });
        }

        print!("Running...");
        None
    }
}

struct GameResult {
    winner: Submission,
    loser: Submission,
    draw: bool,
}

#[post("/tournament")]
pub async fn run_tournament(state: &AppState) -> Result<Json<Scoreboard>, Error> {
    let mut games = vec![];
    let mut scores: HashMap<String, Score> = HashMap::new();

    print!("Generating all possible games...");
    // Generate games
    // NOTE: get_cloned uses lock_value_accessors, which is unstable
    let state = state.get_cloned()?;
    let contestants = state.submissions.values();
    contestants.clone().for_each(|c1|
        contestants.clone().for_each(|c2| {
            if c1 == c2 { return; }
            games.push(AiGame {
                game: Game { checkers: GameState::default(), human_player: Player::White },
                w_player: c1.clone(),
                b_player: c2.clone(),
                turns: 0,
            });
        })
    );
    println!("Done!");

    // Ensure fairness for calculating elo
    let mut rng = SmallRng::from_os_rng();
    games.shuffle(&mut rng);

    println!("Playing games...");
    // Play games
    for mut game in games {
        loop {
            print!("Playing {} vs {}...", game.w_player.name, game.b_player.name);
            let result = game.play().await;
            println!("Done!");

            if let Some(i) = result {
                println!("Winner: {}, loser: {}, draw: {}",i.winner.name, i.loser.name, i.draw);
                let winner_sb = scores.get(&i.winner.name).copied().unwrap_or_default();
                let loser_sb = scores.get(&i.loser.name).copied().unwrap_or_default();

                // Calculate elo
                let (winner_elo, loser_elo) = elo(
                    &winner_sb.elo,
                    &loser_sb.elo,
                    if i.draw { &Outcomes::DRAW } else { &Outcomes::WIN },
                    &EloConfig::default());
                println!("Calculated elo");

                // Generate and insert new scoreboard values
                let (winner_sb,loser_sb) = if i.draw {
                    (Score { draws: winner_sb.draws + 1, elo: winner_elo, ..winner_sb},
                    Score { draws: loser_sb.draws + 1, elo: loser_elo,..loser_sb})
                } else {
                    (Score { wins: winner_sb.wins + 1, elo: winner_elo,..winner_sb},
                    Score { losses: loser_sb.losses + 1, elo: loser_elo,..loser_sb})
                };

                scores.insert(i.winner.name, winner_sb);
                scores.insert(i.loser.name, loser_sb);
                println!("Inserted new scores");
                break;
            }
        }
    }
    println!("DONE PLAYING GAMES");

    Ok(Json(Scoreboard{scores}))
}
