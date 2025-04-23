use async_trait::async_trait;
use chrono::Timelike;
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse};

/**
* filename : eval_transaction_time
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

//✅ 지정 시간대를 벗어나서 사용한 자. (default: 9시~6시, 사용자 별로 설정 가능)
pub struct TransactionTimeEvaluator;

#[async_trait]
impl Evaluator for TransactionTimeEvaluator {
  async fn evaluate(&self, request: FDSRequest) -> FDSResponse {
    let hour = request.transaction.time.hour();
    
    if (9..=17).contains(&hour) {
      // 09:00 ~ 18:00 (9시부터 17:59까지 허용)
      FDSResponse {
        kind: EvaluateKind::TransactionTime,
        result: EvaluateResult::Pass,
        report: "정상 거래 시간".into(),
      }
    } else {
      update_location_active_deny(&request.customer.id).await;
      notify_rabbitmq(&request.customer.id, "거래 시간대 위반").await;
      
      FDSResponse {
        kind: EvaluateKind::TransactionTime,
        result: EvaluateResult::Deny,
        report: "지정 거래 시간을 벗어남".into(),
      }
    }
  }
}

async fn update_location_active_deny(user_id: &str) {
  println!("[DB] location_active = deny for user: {}", user_id);
}

async fn notify_rabbitmq(user_id: &str, msg: &str) {
  println!("[RabbitMQ] [{}] 사용자 {} 에 대한 알림", msg, user_id);
}
