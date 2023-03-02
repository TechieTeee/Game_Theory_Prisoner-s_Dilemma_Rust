extern crate rand;

use rand::Rng;

fn play_game(player1_cooperates: bool, player2_cooperates: bool) -> (i32, i32) {
    let (mut player1_score, mut player2_score) = (0, 0);
    if player1_cooperates {
        if player2_cooperates {
            player1_score = 3;
            player2_score = 3;
        } else {
            player1_score = 0;
            player2_score = 5;
        }
    } else {
        if player2_cooperates {
            player1_score = 5;
            player2_score = 0;
        } else {
            player1_score = 1;
            player2_score = 1;
        }
    }
    (player1_score, player2_score)
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut player1_total_score = 0;
    let mut player2_total_score = 0;
    for _ in 0..100 {
        let player1_cooperates = rng.gen_bool(0.5);
        let player2_cooperates = rng.gen_bool(0.5);
        let (player1_score, player2_score) = play_game(player1_cooperates, player2_cooperates);
        player1_total_score += player1_score;
        player2_total_score += player2_score;
    }

    println!("Player 1 total score: {}", player1_total_score);
    println!("Player 2 total score: {}", player2_total_score);
}
