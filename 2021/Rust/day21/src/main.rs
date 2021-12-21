use simplelog::*;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    println!("Part 1: {}", run(9, 3));
    // println!("Part 2: {}", get_result(&input, &rules, 40));

    Ok(())
}

struct State {
    dice: u32,
    round_count: u32,
}

impl State {
    fn new() -> State {
        State {
            dice: 1,
            round_count: 0,
        }
    }
}

struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn new(initial: u32) -> Player {
        Player {
            position: initial,
            score: 0,
        }
    }
}
fn run(player1_pos: u32, player2_pos: u32) -> u32 {
    fn play_next_round(player: &mut Player, state: &mut State) -> u32 {
        let roll: u32 = (0..3).map(|_| get_roll(state)).sum();
        player.position = (player.position - 1 + roll) % 10 + 1;
        player.score += player.position;
        state.round_count += 3;
        player.score
    }
    fn get_roll(state: &mut State) -> u32 {
        let roll = state.dice;
        state.dice += 1;
        if state.dice > 100 {
            state.dice = 1;
        }
        roll
    }

    let mut player1 = Player::new(player1_pos);
    let mut player2 = Player::new(player2_pos);
    let mut state = State::new();

    let loser = loop {
        if play_next_round(&mut player1, &mut state) >= 1000 {
            break &player2;
        }
        if play_next_round(&mut player2, &mut state) >= 1000 {
            break &player1;
        }
    };
    loser.score * state.round_count
}
