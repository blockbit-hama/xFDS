/**
* filename : eval_money
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

// ✅ 1회 출금 신청 시 한화 ￦100,000,000 (일억만원) 미화 $80,000 이상 출금 요청한자

use async_trait::async_trait;
use std::sync::Arc;
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse};

pub struct MoneyEvaluator;

#[async_trait]
impl Evaluator for MoneyEvaluator {
  async fn evaluate(&self, request: &FDSRequest) -> FDSResponse {
    const LIMIT_KRW: i64 = 100_000_000;
    
    if request.transaction.amount > LIMIT_KRW {
      update_money_active_deny(&request.customer.id).await;
      notify_to_rabbitmq(&request.customer.id, "High amount withdrawal").await;
      
      FDSResponse {
        kind: EvaluateKind::Money,
        result: EvaluateResult::Deny,
        report: "Withdrawal over ￦100,000,000".into(),
      }
    } else {
      FDSResponse {
        kind: EvaluateKind::Money,
        result: EvaluateResult::Pass,
        report: "Normal amount".into(),
      }
    }
  }
}

async fn update_money_active_deny(user_id: &str) {
  println!("🚫 [DB] money_active = deny for user: {}", user_id);
}

async fn notify_to_rabbitmq(user_id: &str, msg: &str) {
  println!("📢 [RabbitMQ] Alert for {}: {}", user_id, msg);
}
