/**
* filename : model
* author : HAMA
* date: 2025. 4. 23.
* description: 
**/

#[derive(Debug, Clone)]
pub enum EvaluateKind {
  Day,
  FirstUsed,
  Repeat,
  Ip,
  Location,
  Money,
  Password,
  Profile,
  Dormant,
  TransactionTime,
}

impl EvaluateKind {
  pub fn description(&self) -> &'static str {
    match self {
      EvaluateKind::Day => "day related evaluation",
      EvaluateKind::FirstUsed => "If you try to withdrawing money for the first time within 24 hours after signing up and deposit",
      EvaluateKind::Repeat => "repeat related evaluation",
      EvaluateKind::Ip => "ip related evaluation",
      EvaluateKind::Location => "location related evaluation",
      EvaluateKind::Money => "money related evaluation",
      EvaluateKind::Password => "password related evaluation",
      EvaluateKind::Profile => "profile related evaluation",
      EvaluateKind::Dormant => "unused long time related evaluation",
      EvaluateKind::TransactionTime => "transaction time related evaluation",
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluateResult {
  Pass,
  Deny,
  Exception,
}

#[derive(Debug, Clone)]
pub struct FDSResponse {
  pub kind: EvaluateKind,
  pub result: EvaluateResult,
  pub report: String,
}


use chrono::{DateTime, Utc};

#[derive(Default, Debug, Clone)]
pub struct CustomerInfo {
  pub id: String,
  pub name: String,
}

#[derive(Default, Debug, Clone)]
pub struct TransactionInfo {
  pub ip: String,
  pub location: String,
  pub amount: i64,
  pub time: DateTime<Utc>,
}

#[derive(Default, Debug, Clone)]
pub struct FDSRequest {
  pub customer: CustomerInfo,
  pub transaction: TransactionInfo,
}
