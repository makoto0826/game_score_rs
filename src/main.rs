mod score;

const LIMIT: u32 = 10;

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let mut scores = match score::get_scores(args.score_path) {
        Ok(scores) => scores,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let players = match score::get_players(args.player_path) {
        Ok(players) => players,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    score::sort(&mut scores);
    let ranking_scores = score::rank(&scores, LIMIT);
    score::output(&ranking_scores, &players);
}

struct Args {
    score_path: String,
    player_path: String,
}

fn get_args() -> Result<Args, String> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err("引数に誤りがあります".to_string());
    }

    Ok(Args {
        score_path: args[1].clone(),
        player_path: args[2].clone(),
    })
}
