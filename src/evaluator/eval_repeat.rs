use async_trait::async_trait;
use chrono::{Duration, Utc};
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse};

/**
* filename : eval_repeat
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

//✅ 60분이내 3회 반복 출금 신청한 자
//✅ 1일간 5회 이상 반복 출금 요청한자

pub struct RepeatEvaluator;

#[async_trait]
impl Evaluator for RepeatEvaluator {
  async fn evaluate(&self, request: &FDSRequest) -> FDSResponse {
    let now = Utc::now();
    
    let fds_status = get_fds_status(&request.customer.id).await;
    
    if fds_status.oneday_freepass_date > now {
      return FDSResponse {
        kind: EvaluateKind::Repeat,
        result: EvaluateResult::Pass,
        report: "Freepass 기간".into(),
      };
    }
    
    let past_hour_tx = get_withdrawals_since(&request.customer.id, now - Duration::minutes(60)).await;
    if past_hour_tx.len() > 2 {
      update_location_active_deny(&request.customer.id).await;
      notify_rabbitmq(&request.customer.id, "1시간 내 3회 이상 출금 시도").await;
      
      return FDSResponse {
        kind: EvaluateKind::Repeat,
        result: EvaluateResult::Deny,
        report: "1시간 내 3회 이상 출금 요청".into(),
      };
    }
    
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let today_tx = get_withdrawals_since(&request.customer.id, chrono::DateTime::from_naive_utc_and_offset(today_start, Utc)).await;
    
    if today_tx.len() >= 5 {
      update_location_active_deny(&request.customer.id).await;
      notify_rabbitmq(&request.customer.id, "하루 5회 이상 출금 시도").await;
      
      return FDSResponse {
        kind: EvaluateKind::Repeat,
        result: EvaluateResult::Deny,
        report: "당일 5회 이상 출금 요청".into(),
      };
    }
    
    FDSResponse {
      kind: EvaluateKind::Repeat,
      result: EvaluateResult::Pass,
      report: "반복 출금 없음".into(),
    }
  }
}

#[derive(Debug)]
pub struct FDSStatus {
  pub oneday_freepass_date: chrono::DateTime<Utc>,
}

#[derive(Debug,Clone)]
pub struct TransactionInfo {
  pub time: chrono::DateTime<Utc>,
  // ... other fields
}


async fn get_fds_status(_id: &str) -> FDSStatus {
  FDSStatus {
    oneday_freepass_date: Utc::now() - Duration::days(1), // 과거
  }
}

async fn get_withdrawals_since(_id: &str, _from: chrono::DateTime<Utc>) -> Vec<TransactionInfo> {
  // 실제 구현은 DB 조회
  vec![TransactionInfo { time: Utc::now() }; 3]
}

async fn update_location_active_deny(user_id: &str) {
  println!("[DB] location_active = deny for user: {}", user_id);
}

async fn notify_rabbitmq(user_id: &str, msg: &str) {
  println!("[RabbitMQ] [{}] 사용자 {} 에 대한 알림", msg, user_id);
}
