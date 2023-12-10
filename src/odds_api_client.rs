pub mod ncaab_totals;
use ncaab_totals::OddsApiNCAABResponse;

pub struct OddsApiClient {
    client: reqwest::Client,
    api_key: String,
    odds_format: Option<String>,
    base_url: String,
}

impl OddsApiClient {
    pub fn new(api_key: &str, odds_format: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            odds_format: odds_format.or(Some(String::from("american"))),
            base_url: "https://api.the-odds-api.com/v4/sports".to_string(),
        }
    }

    pub async fn nccab_totals(
        &self,
        bookmakers: &[&str],
    ) -> Result<Vec<OddsApiNCAABResponse>, reqwest::Error> {
        let bookmakers_param = bookmakers.join(",");

        let params = [
            ("apiKey", &self.api_key),
            ("bookmakers", &bookmakers_param),
            ("markets", &"totals".to_string()),
            ("oddsFormat", &self.odds_format.as_ref().unwrap()),
        ];

        let res = self
            .client
            .get(&format!("{}/basketball_ncaab/odds", self.base_url))
            .query(&params)
            .send()
            .await?;
        let res = res.json::<Vec<OddsApiNCAABResponse>>().await?;
        Ok(res)
    }
}
