/**
* filename : ip_evaluator
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

// ✅ [지정된 ip가 아닌 ip 에서 출금 요청한 경우]
use async_trait::async_trait;
use std::sync::Arc;
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse};

pub struct IPEvaluator;

#[async_trait]
impl Evaluator for IPEvaluator {
  async fn evaluate(&self, request: FDSRequest) -> FDSResponse {
    if request.transaction.ip == request.transaction.ip {
      return FDSResponse {
        kind: EvaluateKind::Ip,
        result: EvaluateResult::Pass,
        report: String::new(),
      };
    }
    
    let user_id = request.customer.id.clone();
    
    // ❌ IP 불일치 → 상태 업데이트 및 알림 전송
    update_ip_active_deny(&user_id).await;
    notify_to_rabbitmq(&user_id, "IP mismatch detected").await;
    
    FDSResponse {
      kind: EvaluateKind::Ip,
      result: EvaluateResult::Deny,
      report: format!(
        "request ip({}) does not match",
        request.transaction.ip
      ),
    }
  }
}

async fn update_ip_active_deny(user_id: &str) {
  println!("Updated fds_status.ip_active = deny for user: {}", user_id);
}

async fn notify_to_rabbitmq(user_id: &str, msg: &str) {
  println!("RabbitMQ alert for user {}: {}", user_id, msg);
}
