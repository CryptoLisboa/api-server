use serde::{Deserialize, Serialize};
use tracing::info;

use crate::types::model::{
    Balance, BalanceWrapper, Chart, ChartWrapper, Coin, Curve, Swap, Thread, ThreadWrapper,
};

use super::{CoinAndUserInfo, CoinInfo};
use super::{NewSwapMessage, NewTokenMessage, SendMessageType, UserInfo};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CoinResponse {
    pub id: String,
    #[serde(rename = "swaps")]
    pub swap: Option<Vec<Swap>>,
    #[serde(rename = "charts")]
    pub chart: Option<Vec<ChartWrapper>>,
    #[serde(rename = "balances")]
    pub balance: Option<Vec<BalanceWrapper>>,
    pub curve: Option<Curve>,

    #[serde(rename = "threads")]
    pub thread: Option<Vec<ThreadWrapper>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoinMessage {
    #[serde(skip)]
    pub message_type: SendMessageType,
    pub new_token: Option<NewTokenMessage>,
    pub new_buy: Option<NewSwapMessage>,
    pub new_sell: Option<NewSwapMessage>,
    pub coin: CoinResponse,
}

impl CoinMessage {
    pub fn from_coin(coin: Coin, info: CoinAndUserInfo) -> Self {
        CoinMessage {
            message_type: SendMessageType::ALL,
            new_token: Some(NewTokenMessage {
                user_info: UserInfo {
                    nickname: info.user_nickname,
                    image_uri: info.user_image_uri,
                },
                symbol: info.coin_symbol,
                image_uri: info.coin_image_uri,
                created_at: coin.created_at,
            }),
            new_buy: None,
            new_sell: None,
            coin: CoinResponse {
                id: coin.id.clone(),
                swap: None,
                chart: None,
                balance: None,
                curve: None,
                thread: None,
            },
        }
    }
    pub fn from_swap(swap: Swap, info: CoinAndUserInfo) -> Self {
        match swap.is_buy {
            true => CoinMessage {
                message_type: SendMessageType::ALL,
                new_token: None,
                new_buy: Some(NewSwapMessage {
                    user_info: UserInfo {
                        nickname: info.user_nickname,
                        image_uri: info.user_image_uri,
                    },
                    is_buy: true,
                    coin_info: CoinInfo {
                        symbol: info.coin_symbol,
                        image_uri: info.coin_image_uri,
                    },
                    nad_amount: swap.nad_amount.to_string(),
                }),
                new_sell: None,
                coin: CoinResponse {
                    id: swap.coin_id.clone(),
                    swap: Some(vec![swap]),
                    chart: None,
                    balance: None,
                    curve: None,
                    thread: None,
                },
            },
            false => CoinMessage {
                message_type: SendMessageType::ALL,
                new_token: None,
                new_buy: None,
                new_sell: Some(NewSwapMessage {
                    user_info: UserInfo {
                        nickname: info.user_nickname,
                        image_uri: info.user_image_uri,
                    },
                    is_buy: false,
                    coin_info: CoinInfo {
                        symbol: info.coin_symbol,
                        image_uri: info.coin_image_uri,
                    },
                    nad_amount: swap.nad_amount.to_string(),
                }),
                coin: CoinResponse {
                    id: swap.coin_id.clone(),
                    swap: Some(vec![swap]),
                    chart: None,
                    balance: None,
                    curve: None,
                    thread: None,
                },
            },
        }
    }
    pub fn from_chart(chart: ChartWrapper) -> Self {
        CoinMessage {
            message_type: SendMessageType::Regular,
            new_token: None,
            new_buy: None,
            new_sell: None,
            coin: CoinResponse {
                id: chart.coin_id.clone(),
                swap: None,
                chart: Some(vec![chart]),
                balance: None,
                curve: None,
                thread: None,
            },
        }
    }
    pub fn from_balance(balance: BalanceWrapper) -> Self {
        CoinMessage {
            message_type: SendMessageType::Regular,
            new_token: None,
            new_buy: None,
            new_sell: None,
            coin: CoinResponse {
                id: balance.coin_id.clone(),
                swap: None,
                chart: None,
                balance: Some(vec![balance]),
                curve: None,
                thread: None,
            },
        }
    }
    pub fn from_curve(curve: Curve) -> Self {
        CoinMessage {
            message_type: SendMessageType::Regular,
            new_token: None,
            new_buy: None,
            new_sell: None,
            coin: CoinResponse {
                id: curve.coin_id.clone(),
                swap: None,
                chart: None,
                balance: None,
                curve: Some(curve),
                thread: None,
            },
        }
    }

    pub fn from_thread(thread: ThreadWrapper) -> Self {
        CoinMessage {
            message_type: SendMessageType::Regular,
            new_token: None,
            new_buy: None,
            new_sell: None,
            coin: CoinResponse {
                id: thread.coin_id.clone(),
                swap: None,
                chart: None,
                balance: None,
                curve: None,
                thread: Some(vec![thread]),
            },
        }
    }
}
