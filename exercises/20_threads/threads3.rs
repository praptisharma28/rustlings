use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default, Debug)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // Get or create TeamScores for team 1
        let team_1 = scores.entry(team_1_name).or_insert(TeamScores::default());
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;  // Team 2's goals = Team 1's conceded

        // Get or create TeamScores for team 2  
        let team_2 = scores.entry(team_2_name).or_insert(TeamScores::default());
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;  // Team 1's goals = Team 2's conceded
    }

    scores
}

fn main() {
    // Let's test with the example data
    let results = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    let scores = build_scores_table(results);
    
    println!("Soccer Scores Table:");
    println!("{:<10} {:<6} {:<6}", "Team", "Scored", "Conceded");
    println!("{:-<24}", "");
    
    for (team, stats) in &scores {
        println!("{:<10} {:<6} {:<6}", team, stats.goals_scored, stats.goals_conceded);
    }
    
    // Let's trace England's matches:
    println!("\nEngland's matches:");
    println!("vs France: scored 4, conceded 2");
    println!("vs Germany: scored 1, conceded 2"); 
    println!("vs Spain: scored 1, conceded 0");
    println!("Total: scored 6, conceded 4");
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
