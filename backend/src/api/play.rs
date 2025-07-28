use super::{submissions::Submission, AppState, Error, User};
use crate::{
    game::{GameState, GameStatus, Move, Player, TurnStatus}};
use rocket::{
    futures::{io::BufReader, AsyncReadExt, AsyncWriteExt},
    get, post,
    serde::json::Json,
    tokio::sync::Mutex,
};
use std::{
    sync::{Arc},
    os::unix::net::UnixListener,
    io::Read,
};

#[derive(Debug)]
pub struct Game {
    checkers: GameState,
    human_player: Player,
}

fn convert_cell_id(id: &[char]) -> (usize, usize) {
    (id[0] as usize - '0' as usize, id[1] as usize - '0' as usize)
}

struct AiOutput {
    move_: String,
    console: String,
}

impl Game {
    async fn play_ai(&mut self, submission: Submission) -> Result<AiOutput, Error> {
        // We use UNIX sockets for communication with the submission scripts
        // There is one socket per user

        // Clean up old sockets and prepare a new one
        // NOTE: Are we sure /tmp will always exist?
        let socket_adr = format!("/tmp/ai_{}.sock", submission.name);
        let _ = std::fs::remove_file(&socket_adr);

        let mut child = submission.start(socket_adr.clone()).await?;

        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let mut stderr = BufReader::new(child.stderr.take().unwrap());
        let listener = UnixListener::bind(socket_adr)?;

        stdin
            .write_all(format!("{}\n", self.checkers.current_player).as_bytes())
            .await
            .map_err(Error::from)?;

        stdin
            .write_all(self.checkers.to_csv_string().as_bytes())
            .await
            .map_err(Error::from)?;
        child.status().await?;

        let mut out = String::new();
        stdout.read_to_string(&mut out).await?;

        let mut err = String::new();
        stderr.read_to_string(&mut err).await?;
        let ai_output = out + &err;

        //println!("output for user {}:\"{}\"", submission.name, ai_output);
        // Read socket
        let mut mov = String::new();
        match listener.accept() {
            Ok((mut socket, _)) => {
                socket.read_to_string(&mut mov)?;
            },
            Err(e) => println!("Failed to accept socket connection: {}", e),
        }

        let seq = mov
            .split(";")
            .filter(|m| !m.is_empty())
            .map(|m| {
                let chars = m.chars().collect::<Vec<_>>();
                Move {
                    from: convert_cell_id(&chars[0..=1]),
                    to: convert_cell_id(&chars[3..=4]),
                }
            })
            .collect::<Vec<_>>();

        if let Err(Error::InvalidMove) = self.checkers.apply_sequence(&seq) {
            self.checkers.status = GameStatus::Victory(self.human_player);
            return Err(Error::AIFailed {
                error: super::AIError::InvalidMove,
                ai_output,
                move_: Some(seq),
            });
        }

        Ok(AiOutput{ move_: mov, console: ai_output })
    }

    pub async fn play_human(&mut self, moves: Vec<Move>) -> Result<(), Error> {
        self.checkers.apply_sequence(&moves)
    }
}

#[get("/game")]
pub async fn get_game(state: &AppState, user: User) -> Result<Json<GameState>, Error> {
    let game = {
        let mutex = {
            let lock = state.lock()?;
            lock.games.get(&user.name).ok_or(Error::NotFound)?.clone()
        };

        let lock = mutex.lock().await;
        lock.checkers.clone()
    };

    Ok(Json(game))
}

#[post("/game/start?<is_first_player>")]
pub async fn start(
    state: &AppState,
    user: User,
    is_first_player: bool,
) -> Result<Json<TurnStatus>, Error> {
    let checkers: GameState = Default::default();

    let mut game = Game {
        human_player: if is_first_player {
            Player::White
        } else {
            Player::Black
        },
        checkers: checkers.clone(),
    };

    let mut ai_move = String::new();
    let mut console = String::new();
    if !is_first_player {
        let submission = state
            .lock()
            .unwrap()
            .submissions
            .get(&user.name)
            .ok_or(Error::NotFound)?
            .clone();

        AiOutput { move_: ai_move, console: console } = game.play_ai(submission).await?;
    }

    println!("console output: {}", console);
    let checkers = game.checkers.clone();

    let mut lock = state.lock().unwrap();
    lock.games.insert(user.name, Arc::new(Mutex::new(game)));

    Ok(Json(TurnStatus {
        game: checkers,
        move_: ai_move,
        ai_output: console,
    }))
}

#[post("/game", format = "json", data = "<moves>")]
pub async fn play(
    state: &AppState,
    user: User,
    moves: Json<Vec<Move>>,
) -> Result<Json<TurnStatus>, Error> {
    let submission = state
        .lock()
        .unwrap()
        .submissions
        .get(&user.name)
        .ok_or(Error::NotFound)?
        .clone();
    let game = state.lock().unwrap().games.get(&user.name).map(Arc::clone);

    if game.is_none() {
        return Err(Error::NotFound);
    }

    let game = game.unwrap();
    let mut lock = game.lock().await;

    lock.play_human(moves.into_inner()).await?;
    let output = lock.play_ai(submission).await?;

    Ok(Json(TurnStatus {
        game: lock.checkers.clone(),
        ai_output: output.console,
        move_: output.move_,
    }))
}

#[post("/game/stop")]
pub async fn stop(state: &AppState, user: User) -> Result<(), Error> {
    let game = state.lock().unwrap().games.remove(&user.name);

    if game.is_none() {
        return Err(Error::NotFound);
    }

    Ok(())
}
