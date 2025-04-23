/**
* filename : Evaludator
* author : HAMA
* date: 2025. 4. 15.
* description:
**/

use crate::evaluator::model::{FDSRequest, FDSResponse};

#[async_trait::async_trait]
pub trait Evaluator {
  async fn evaluate(&self, request: FDSRequest)-> FDSResponse;
}

