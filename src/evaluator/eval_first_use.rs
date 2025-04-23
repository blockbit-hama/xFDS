/**
* filename : first_used_evaluator
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

use chrono::{DateTime, Duration, Utc};
use async_trait::async_trait;
use std::sync::Arc;
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse};

// 가상의 상태
#[derive(Debug)]
pub struct FDSStatus {
  pub first_try: bool,
}

// [가입 후 입금 시도한 뒤 24시간내 첫 출금 시도할 경우]
pub struct FirstUsedEvaluator;

#[async_trait]
impl Evaluator for FirstUsedEvaluator {
  async fn evaluate(&self,request: FDSRequest) -> FDSResponse {
    let user_id = request.customer.id.clone();
    
    let mut status = get_fds_status(&user_id).await;
    
    if status.first_try {
      // ✅ 첫 시도 상태 갱신
      status.first_try = false;
      update_fds_status(&user_id, &status).await;
      
      // ✅ 입금 및 출금 시각 확인
      let deposit = get_oldest_deposit(&user_id).await;
      let withdrawal = get_first_withdrawal(&user_id).await;
      
      if let (Some(deposit_time), Some(withdrawal_time)) = (deposit, withdrawal) {
        let duration = withdrawal_time - deposit_time;
        
        if duration < Duration::hours(24) {
          // 조건 충족: DENY
          update_first_try_active(&user_id, false).await;
          send_alert_to_rabbitmq(&user_id, "FIRST_USED DENY").await;
          
          return FDSResponse {
            kind: EvaluateKind::FirstUsed,
            result: EvaluateResult::Deny,
            report: String::new(),
          };
        }
      }
      
      // 조건 불충족: PASS
      return FDSResponse {
        kind: EvaluateKind::FirstUsed,
        result: EvaluateResult::Pass,
        report: String::new(),
      };
    }
    
    // 이미 평가됨
    FDSResponse {
      kind: EvaluateKind::FirstUsed,
      result: EvaluateResult::Pass,
      report: String::new(),
    }
  }
}

async fn get_fds_status(_user_id: &str) -> FDSStatus {
  FDSStatus { first_try: true }
}

async fn update_fds_status(_user_id: &str, _status: &FDSStatus) {
  println!("🔧 fds_status updated");
}

async fn update_first_try_active(_user_id: &str, _deny: bool) {
  println!("first_try_active = deny");
}

async fn get_oldest_deposit(_user_id: &str) -> Option<DateTime<Utc>> {
  Some(Utc::now() - Duration::hours(23))
}

async fn get_first_withdrawal(_user_id: &str) -> Option<DateTime<Utc>> {
  Some(Utc::now())
}

async fn send_alert_to_rabbitmq(user_id: &str, message: &str) {
  println!("Alert to RabbitMQ: [{}] {}", user_id, message);
}
