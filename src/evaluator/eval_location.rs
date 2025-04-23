/**
* filename : eval_location
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

use async_trait::async_trait;
use std::sync::Arc;
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::model::{EvaluateKind, EvaluateResult, FDSRequest, FDSResponse};

// ✅ 국외에서 출금 요청한 자.(평상시 사용한 위치를 벗어나서 타 지역에서 출금 요청한 자)
pub struct LocationEvaluator;

impl LocationEvaluator {
  pub const LOCATION_SUCCESS: &'static str = "location success";
  pub const LOCATION_FAILURE: &'static str = "location failure";
  pub const LOCATION_EXCEPTION: &'static str = "location exception";
  
}

#[async_trait]
impl Evaluator for LocationEvaluator {
  async fn evaluate(&self, request: FDSRequest) -> FDSResponse {
    let result = std::panic::AssertUnwindSafe(async {
      let is_foreign = !request.transaction.location.to_lowercase().contains("korea");
      
      if is_foreign {
        update_location_active_deny(&request.customer.id).await;
        notify_to_rabbitmq(&request.customer.id, "location mismatch").await;
        self.log_message(&request.customer, &request.transaction.location, Self::LOCATION_FAILURE);
        
        FDSResponse {
          kind: EvaluateKind::Location,
          result: EvaluateResult::Deny,
          report: Self::LOCATION_FAILURE.into(),
        }
      } else {
        self.log_message(&request.customer, &request.transaction.location, Self::LOCATION_SUCCESS);
        
        FDSResponse {
          kind: EvaluateKind::Location,
          result: EvaluateResult::Pass,
          report: Self::LOCATION_SUCCESS.into(),
        }
      }
    })
      .catch_unwind()
      .await;
    
    match result {
      Ok(resp) => resp,
      Err(e) => {
        FDSResponse {
          kind: EvaluateKind::Location,
          result: EvaluateResult::Exception,
          report: format!("{:?}", e),
        }
      }
    }
  }
}

async fn update_location_active_deny(user_id: &str) {
  println!("Updated fds_status.location_active = deny for user: {}", user_id);
}

async fn notify_to_rabbitmq(user_id: &str, msg: &str) {
  println!("[RabbitMQ] Alert for {}: {}", user_id, msg);
}
