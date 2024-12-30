use crate::{
    client::{ create_jwt, Client },
    http_method::HttpMethod,
    portfolio_type::PortfolioType,
    portfolios::Portfolios,
};

pub struct ApiPortfolios;

impl ApiPortfolios {
    pub async fn list_portfolios(
        client: &Client<'_>,
        portfolio_type: Option<PortfolioType>
    ) -> Result<Portfolios, reqwest::Error> {
        let portfolio_type = match portfolio_type {
            Some(portfolio_type) => &format!("?portfolio_type={}", portfolio_type),
            None => "",
        };
        let url = &format!("{}{}", PORTFOLIOS_URL, portfolio_type);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), PORTFOLIOS_ENDPOINT)
        ).await?;
        let portfolios: Portfolios = response.json().await?;
        Ok(portfolios)
    }
}

const PORTFOLIOS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/portfolios";
const PORTFOLIOS_ENDPOINT: &str = "/api/v3/brokerage/portfolios";
