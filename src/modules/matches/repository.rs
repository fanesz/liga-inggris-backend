use super::entity::{Match, MatchReq, MatchStats};
use crate::common::utils::{compare::compare_lcase, dummy::read_dummy};

pub fn get_matches(req: Option<MatchReq>) -> Vec<Match> {
    read_dummy::<Vec<Match>>("matches/matches.json")
        .into_iter()
        .filter(|_match| match &req {
            Some(filter) => {
                let matches_club = match filter.club_id {
                    Some(id) => _match.home_club_id == id || _match.away_club_id == id,
                    None => true,
                };
                let matches_status = match &filter.status {
                    Some(status) => compare_lcase(&_match.status, &status),
                    None => true,
                };
                matches_club && matches_status
            }
            None => true,
        })
        .collect()
}

pub fn get_match_by_id(id: u64) -> Option<Match> {
    get_matches(None).into_iter().find(|_match| _match.id == id)
}

pub fn get_match_stats(match_id: u64) -> Option<MatchStats> {
    read_dummy::<Vec<MatchStats>>("matches/match-stats.json")
        .into_iter()
        .find(|m| m.match_id == match_id)
}
