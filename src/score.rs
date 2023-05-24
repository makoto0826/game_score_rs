use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct Player {
    player_id: String,
    handle_name: String,
}

impl Player {
    fn new<T: Into<String>>(player_id: T, handle_name: T) -> Self {
        Self {
            player_id: player_id.into(),
            handle_name: handle_name.into(),
        }
    }
}

#[derive(Debug)]
pub struct Score {
    player_id: String,
    total_score: f64,
    mean_score: f64,
    play_count: u32,
}

impl Score {
    fn new<T: Into<String>>(player_id: T, score: f64) -> Self {
        Self {
            player_id: player_id.into(),
            total_score: score,
            mean_score: 0.0,
            play_count: 1,
        }
    }

    fn add(&mut self, score: f64) {
        self.total_score += score;
        self.play_count += 1;
    }

    fn average(&mut self) {
        if self.play_count != 0 {
            self.mean_score = (self.total_score / self.play_count as f64).round();
        }
    }
}

#[derive(Debug)]
pub struct RankingScore<'a> {
    pub rank: u32,
    pub inner: &'a Score,
}

///
/// プレイヤーファイルからプレイヤーを取得します。
///
/// player_path - プレイヤーファイル
///
pub fn get_players<P: AsRef<Path>>(player_path: P) -> Result<HashMap<String, Player>, String> {
    let file = match File::open(player_path) {
        Ok(file) => file,
        Err(_) => {
            return Err("プレイヤーファイルのオープンに失敗しました".to_string());
        }
    };

    let mut reader = BufReader::new(file);
    let _ = reader.read_line(&mut String::new());
    let mut players: HashMap<String, Player> = HashMap::new();

    loop {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let fields: Vec<&str> = line.lines().collect::<Vec<&str>>()[0].split(",").collect();

                if fields.len() != 2 {
                    return Err("プレイヤーファイルのフォーマットが不正です".to_string());
                }

                let player_id = fields[0].to_string();
                let handle_name = fields[1].to_string();

                players.insert(player_id.clone(), Player::new(player_id, handle_name));
            }
            Err(_) => {
                return Err("スコアファイルの読み取りに失敗しました".to_string());
            }
        }
    }

    Ok(players)
}

///
/// スコアファイルからプレイヤー単位毎のスコア情報を取得します。
///
/// score_path - スコアファイル
///
pub fn get_scores<P: AsRef<Path>>(score_path: P) -> Result<Vec<Score>, String> {
    let file = match File::open(score_path) {
        Ok(file) => file,
        Err(_) => {
            return Err("スコアファイルのオープンに失敗しました".to_string());
        }
    };

    let mut reader = BufReader::new(file);
    let _ = reader.read_line(&mut String::new());
    let mut scores: HashMap<String, Score> = HashMap::new();

    loop {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let fields: Vec<&str> = line.lines().collect::<Vec<&str>>()[0].split(",").collect();

                if fields.len() != 3 {
                    return Err("スコアファイルのフォーマットが不正です".to_string());
                }

                let player_id = fields[1].to_string();
                let Ok(score) = fields[2].parse::<f64>() else {
                    return Err("スコアがフォーマットが不正です".to_string());
                };

                match scores.get_mut(&player_id) {
                    Some(score_info) => score_info.add(score),
                    None => {
                        scores.insert(player_id.clone(), Score::new(&player_id, score));
                    }
                }
            }
            Err(_) => {
                return Err("スコアファイルの読み取りに失敗しました".to_string());
            }
        }
    }

    for score in scores.values_mut() {
        score.average();
    }

    Ok(scores.into_values().collect())
}

///
/// スコア情報を並び返します。
///
/// scores - スコア情報
///
pub fn sort(scores: &mut Vec<Score>) {
    scores.sort_by(|a, b| {
        if a.mean_score == b.mean_score {
            if a.player_id > b.player_id {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            };
        }

        if a.mean_score < b.mean_score {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
}

///
/// 平均したスコアから順位を付与します。
///
/// scores - スコア情報
///
/// limit - 順位を付与する上限
///
pub fn rank(scores: &Vec<Score>, limit: u32) -> Vec<RankingScore> {
    let mut rank = 1;
    let mut index = 0;
    let mut ranking_scores = Vec::<RankingScore>::new();

    while scores.len() > index {
        ranking_scores.push(RankingScore {
            rank,
            inner: &scores[index],
        });

        let next_index = index + 1;

        if scores.len() > next_index {
            rank = if scores[index].mean_score == scores[next_index].mean_score {
                rank
            } else {
                rank + 1
            };
        }

        if rank > limit {
            break;
        }

        index += 1;
    }

    ranking_scores
}

///
/// ランキングスコアを出力します。
///
/// ranking_scores - ランキングスコア
///
/// players - プレイヤー情報
///
pub fn output(ranking_scores: &Vec<RankingScore>, players: &HashMap<String, Player>) {
    println!("rank,player_id,handle_name,mean_score");

    for score in ranking_scores.iter() {
        let player = players.get(&score.inner.player_id).unwrap();

        println!(
            "{},{},{},{}",
            score.rank, score.inner.player_id, player.handle_name, score.inner.mean_score
        );
    }
}