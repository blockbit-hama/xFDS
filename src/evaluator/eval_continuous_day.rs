/**
* filename : continuous_day
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use async_trait::async_trait;
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse, TransactionInfo};

#[derive(Debug)]
pub struct FDSStatus {
  pub continuous_day_freepass_date: Option<DateTime<Utc>>,
}


// [연속 일자 출금 검증]
// 5일간, 매일 1회 이상 출금 그리고 매일 평균 출금 금액이 1000$ 이상인 유저
pub struct ContinuousDayEvaluator;

#[async_trait]
impl Evaluator for ContinuousDayEvaluator {
  async fn evaluate(&self, request: &FDSRequest) -> FDSResponse {
    let customer_id = &request.customer.id;
    let now = &request.transaction.time;
    
    let fds_status = get_fds_status(customer_id).await;
    
    if let Some(date) = fds_status.continuous_day_freepass_date {
      if date > *now {
        return FDSResponse {
          kind: EvaluateKind::Repeat,
          result: EvaluateResult::Pass,
          report: String::new(),
        };
      }
    }
    
    let transactions = get_recent_transactions(customer_id, *now - Duration::days(1)).await;
    
    let mut daily_totals: HashMap<String, i64> = HashMap::new();
    for tx in transactions {
      let date_str = tx.time.format("%Y-%m-%d").to_string();
      *daily_totals.entry(date_str).or_insert(0) += tx.amount;
    }
    
    let valid_days = daily_totals.iter().filter(|(_, &amount)| amount >= 1000).count();
    
    if valid_days == 5 {
      deny_continuous_day_active(customer_id).await;
      send_alert_to_rabbitmq(customer_id).await;
      return FDSResponse {
        kind: EvaluateKind::Day,
        result: EvaluateResult::Deny,
        report: "5-day high withdrawal pattern detected".to_string(),
      };
    }
    
    FDSResponse {
      kind: EvaluateKind::Day,
      result: EvaluateResult::Pass,
      report: String::new(),
    }
  }
}

// Stubbed functions

async fn get_fds_status(_id: &str) -> FDSStatus {
  FDSStatus {
    continuous_day_freepass_date: None,
  }
}

async fn get_recent_transactions(_id: &str, _from: DateTime<Utc>) -> Vec<TransactionInfo> {
  vec![]
}

async fn deny_continuous_day_active(_id: &str) {
  // update fds_status table
}

async fn send_alert_to_rabbitmq(_id: &str) {
  // send message to queue
}

