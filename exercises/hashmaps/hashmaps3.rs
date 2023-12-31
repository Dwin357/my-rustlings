// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;
use std::collections::hash_map::Entry; // needed for try-4 impl

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}


fn update_table_try2(name: String, own_score: &u8, other_score: &u8, mut table: HashMap<String, Team>) {
    let before :&Team = table.get(&name).unwrap_or(&Team{goals_scored: 0, goals_conceded: 0});
    let after = Team{
        goals_scored: before.goals_scored + *own_score,
        goals_conceded: before.goals_conceded + *other_score
    };
    table.insert(name, after);    
}

// Note: the leading '_ is for the lifetime term
fn update_table_try4(mut entry: Entry<'_, String, Team>, own_score: u8, other_socre: u8) {
    entry
        .and_modify(|t| { t.goals_scored += own_score; t.goals_conceded += other_socre })
        .or_insert(Team{ goals_scored: own_score, goals_conceded: other_socre});
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();        
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        /*
        // This all should be a method :: try 1
        // notice also, the &Team type on the t1 is optional per t2
        let t1 :&Team = scores.get(&team_1_name).unwrap_or(&Team{goals_scored: 0, goals_conceded: 0});
        let t1u = Team{
            goals_scored: t1.goals_scored + team_1_score,
            goals_conceded: t1.goals_conceded + team_2_score
        };
        scores.insert(team_1_name, t1u);

        let t2 = scores.get(&team_2_name).unwrap_or(&Team{goals_scored: 0, goals_conceded: 0});
        let t2u = Team{
            goals_scored: t2.goals_scored + team_2_score,
            goals_conceded: t2.goals_conceded + team_1_score
        };
        scores.insert(team_2_name, t2u);
        */

        // Try 2, with methods!!!
        //// This is all kinds of angry, b/c the ownership of the scores map is a mess (going in and out of method call in loop)
        //// ...realistically, this is something I know is bad design & it is interesting I wasn't allowed to do it :-P
        /*
        update_table_try2(team_1_name, &team_1_score, &team_2_score, scores);
        update_table_try2(team_2_name, &team_2_score, &team_1_score, scores);
        */

        // Try 3, Lambdas
        //// So I was thinking how I would solve this in closure without the messed up ownership (which I have mentally labeled
        //// the "sewing machine problem"), and the solution I came up with was for the map to have a "mutate" method which takes
        //// a [key, fn] where the fn takes as an arg optional<entry-value>.  Upon being called, this method 
        //// - retreives the current value in the map at the key
        //// - passes retreived value to lambda
        //// - replaces map-entry with return value of lambda
        //// Conceptually, this is just like what `collection.map{ |x| x.do_thing() }` is doing, only at a single entry w/ key-access
        // Possible refs
        ////  www.knowbe4.com/careers/blogs/engineering/on-rusts-map-entry-pattern
        // Kind of a spiffy 1 liner way of doing it, but this is still repeating code...
        /*
        scores
            .entry(team_1_name)
            .and_modify(|t| { t.goals_scored += team_1_score; t.goals_conceded += team_2_score })
            .or_insert(Team{
                                goals_scored: team_1_score,
                                goals_conceded: team_2_score});


        scores
            .entry(team_2_name)
            .and_modify(|t| { t.goals_scored += team_2_score; t.goals_conceded += team_1_score })
            .or_insert(Team{
                                goals_scored: team_2_score,
                                goals_conceded: team_1_score});
        */
        

        // Try 4, like 3 but with a method call for dry-ness
        update_table_try4(scores.entry(team_1_name), team_1_score, team_2_score);
        update_table_try4(scores.entry(team_2_name), team_2_score, team_1_score);

    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
