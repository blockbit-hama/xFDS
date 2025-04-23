/**
* filename : eval_unuse
* author : HAMA
* date: 2025. 4. 23.
* description:
**/

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse};

//✅ 장기 미사용( 3개월간 로그인이 없었던 ) 자가 출금을 요청한 경우

pub struct UnusedEvaluator;

#[async_trait]
impl Evaluator for UnusedEvaluator {
  async fn evaluate(&self, _request: FDSRequest) -> FDSResponse {
    let usage_info = get_customer_usage_info(&_request.customer.id).await;
    
    let three_months_ago = Utc::now() - Duration::days(90);
    
    if usage_info.last_login < three_months_ago {
      update_location_active_deny(&_request.customer.id).await;
      notify_rabbitmq(&_request.customer.id, "3개월 이상 미사용 계정").await;
      
      FDSResponse {
        kind: EvaluateKind::Unused,
        result: EvaluateResult::Deny,
        report: "최근 3개월간 사용 내역 없음".into(),
      }
    } else {
      FDSResponse {
        kind: EvaluateKind::Unused,
        result: EvaluateResult::Pass,
        report: "최근 로그인 기록 확인됨".into(),
      }
    }
  }
}

// 사용 정보 (DB에서 조회된 값으로 가정)
#[derive(Debug)]
pub struct CustomerUsageInfo {
  pub last_login: DateTime<Utc>,
}

async fn get_customer_usage_info(_user_id: &str) -> CustomerUsageInfo {
  // 실제로는 DB에서 조회
  CustomerUsageInfo {
    last_login: Utc::now() - Duration::days(120), // 테스트: 4개월 전 로그인
  }
}

async fn update_location_active_deny(user_id: &str) {
  println!("[DB] location_active = deny for user: {}", user_id);
}

async fn notify_rabbitmq(user_id: &str, msg: &str) {
  println!("[RabbitMQ] 알림 전송 - {}: {}", user_id, msg);
}

