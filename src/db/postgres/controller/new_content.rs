use anyhow::Result;
use tracing::info;

use std::sync::Arc;

use crate::{
    db::postgres::PostgresDatabase,
    types::event::{CoinAndUserInfo, CoinInfo, NewSwapMessage, NewTokenMessage, UserInfo},
};

pub struct InitContentController {
    pub db: Arc<PostgresDatabase>,
}

impl InitContentController {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        InitContentController { db }
    }

    pub async fn get_latest_buy(&self) -> Result<Option<NewSwapMessage>> {
        let result = sqlx::query!(
            r#"
            SELECT
                s.is_buy,
                s.nad_amount,
                a.nickname as user_nickname,
                a.image_uri as user_image_uri,
                c.symbol as coin_symbol,
                c.image_uri as coin_image_uri
            FROM swap s
            JOIN account a ON s.sender = a.id
            JOIN coin c ON s.coin_id = c.id
            WHERE s.is_buy = true
            ORDER BY s.created_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(result.map(|row| NewSwapMessage {
            user_info: UserInfo {
                nickname: row.user_nickname,
                image_uri: row.user_image_uri,
            },
            coin_info: CoinInfo {
                symbol: row.coin_symbol,
                image_uri: row.coin_image_uri,
            },
            is_buy: row.is_buy,
            nad_amount: row.nad_amount.to_string(),
        }))
    }

    pub async fn get_latest_sell(&self) -> Result<Option<NewSwapMessage>> {
        let result = sqlx::query!(
            r#"
            SELECT
                s.is_buy,
                s.nad_amount,
                a.nickname as user_nickname,
                a.image_uri as user_image_uri,
                c.symbol as coin_symbol,
                c.image_uri as coin_image_uri
            FROM swap s
            JOIN account a ON s.sender = a.id
            JOIN coin c ON s.coin_id = c.id
            WHERE s.is_buy = false
            ORDER BY s.created_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(result.map(|row| NewSwapMessage {
            user_info: UserInfo {
                nickname: row.user_nickname,
                image_uri: row.user_image_uri,
            },
            coin_info: CoinInfo {
                symbol: row.coin_symbol,
                image_uri: row.coin_image_uri,
            },
            is_buy: row.is_buy,
            nad_amount: row.nad_amount.to_string(),
        }))
    }

    pub async fn get_latest_new_token(&self) -> Result<Option<NewTokenMessage>> {
        let result = sqlx::query!(
            r#"
            SELECT 
                c.symbol,
                c.image_uri as coin_image_uri,
                c.created_at,
                a.nickname as user_nickname,
                a.image_uri as user_image_uri
            FROM coin c
            JOIN account a ON c.creator = a.id
            ORDER BY c.created_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(result.map(|row| NewTokenMessage {
            user_info: UserInfo {
                nickname: row.user_nickname,
                image_uri: row.user_image_uri,
            },
            symbol: row.symbol,
            image_uri: row.coin_image_uri,
            created_at: row.created_at,
        }))
    }
}
