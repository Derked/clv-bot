pub mod odds_api_client;
use odds_api_client::ncaab_totals::OddsApiNCAABTotalsRes;
use odds_api_client::OddsApiClient;

pub async fn arb_search(
    client: &OddsApiClient,
    bookmakers: &[&str],
) -> Result<OddsApiNCAABTotalsRes, reqwest::Error> {
    let res = client.nccab_totals(bookmakers).await?;
    Ok(OddsApiNCAABTotalsRes(res))
}
