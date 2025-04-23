/**
* filename : Evaludator
* author : HAMA
* date: 2025. 4. 15.
* description:
**/

use std::sync::Arc;
use futures::future::join_all;

use crate::evaluator::model::{FDSRequest, FDSResponse};

#[async_trait::async_trait]
pub trait Evaluator {
  async fn evaluate(&self, request: &FDSRequest)-> FDSResponse;
}

// 평가기 실행자
pub async fn evaluate_all(
  evaluators: Vec<Arc<dyn Evaluator>>,
  request: &FDSRequest,
) -> Vec<FDSResponse> {
  let futures = evaluators
    .into_iter()
    .map(|ev| {
      // let req = request.clone();
      async move { ev.evaluate(request).await }
    });
  
  join_all(futures).await
}

