use serde::{ Deserialize, Serialize };

use crate::{
    limit_limit_fok::LimitLimitFok,
    limit_limit_gtc::LimitLimitGtc,
    limit_limit_gtd::LimitLimitGtd,
    market_market_ioc::MarketMarketIoc,
    sor_limit_ioc::SorLimitIoc,
    stop_limit_stop_limit_gtc::StopLimitStopLimitGtc,
    stop_limit_stop_limit_gtd::StopLimitStopLimitGtd,
    trigger_bracket_gtc::TriggerBracketGtc,
    trigger_bracket_gtd::TriggerBracketGtd,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfiguration {
    pub market_market_ioc: Option<MarketMarketIoc>,
    pub sor_limit_ioc: Option<SorLimitIoc>,
    pub limit_limit_gtc: Option<LimitLimitGtc>,
    pub limit_limit_gtd: Option<LimitLimitGtd>,
    pub limit_limit_fok: Option<LimitLimitFok>,
    pub stop_limit_stop_limit_gtc: Option<StopLimitStopLimitGtc>,
    pub stop_limit_stop_limit_gtd: Option<StopLimitStopLimitGtd>,
    pub trigger_bracket_gtc: Option<TriggerBracketGtc>,
    pub trigger_bracket_gtd: Option<TriggerBracketGtd>,
}
