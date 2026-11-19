//! An AWS Lambda function that adds two numbers using the `budget-ledger`
//! library.
//!
//! It receives JSON like `{"left": 2, "right": 3}` and answers with
//! `{"sum": 5}`.

use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};

/// The JSON the function receives.
#[derive(Deserialize)]
struct Request {
    left: u64,
    right: u64,
}

/// The JSON the function answers with.
#[derive(Serialize, Debug, PartialEq)]
struct Response {
    sum: u64,
}

/// The actual work: add the two numbers with the library.
///
/// This is a plain function with no Lambda types in it, so it is easy to test.
fn add_numbers(request: Request) -> Response {
    let sum = budget_ledger::add(request.left, request.right);
    Response { sum }
}

/// Called by the Lambda runtime once for each invocation.
async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    Ok(add_numbers(event.payload))
}

/// The program's starting point. It hands `handler` to the Lambda runtime,
/// which then waits for invocations and calls it for each one.
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_the_two_numbers() {
        let request = Request { left: 2, right: 3 };

        let response = add_numbers(request);

        assert_eq!(response, Response { sum: 5 });
    }

    #[test]
    fn reads_the_request_from_json() {
        let request: Request = serde_json::from_str(r#"{"left": 2, "right": 3}"#).unwrap();

        let response = add_numbers(request);

        assert_eq!(serde_json::to_string(&response).unwrap(), r#"{"sum":5}"#);
    }
}
