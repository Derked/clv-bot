use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct OddsApiNCAABResponse {
    id: String,
    sport_key: String,
    sport_title: String,
    commence_time: String,
    home_team: String,
    away_team: String,
    bookmakers: Vec<Bookmaker>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmaker {
    key: String,
    title: String,
    last_update: String,
    markets: Vec<Market>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Market {
    key: String,
    last_update: String,
    outcomes: Vec<Outcome>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Outcome {
    name: String,
    price: i32,
    point: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OddsApiNCAABTotalsRes(pub Vec<OddsApiNCAABResponse>);

impl fmt::Display for OddsApiNCAABResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Example format, customize it according to your needs
        write!(
            f,
            "Sport: {} Home Team: {} Away Team: {}\n",
            self.sport_title, self.home_team, self.away_team
        )?;

        for bookmaker in &self.bookmakers {
            write!(f, "\nBookmaker: {} ", bookmaker.title)?;
            for market in &bookmaker.markets {
                for outcome in &market.outcomes {
                    write!(
                        f,
                        "  - {}: {} @ {}\n",
                        outcome.name, outcome.point, outcome.price
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for OddsApiNCAABTotalsRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for game in &self.0 {
            write!(f, "{}", game)?;
        }
        Ok(())
    }
}
