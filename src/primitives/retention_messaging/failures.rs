use std::collections::HashMap;
use crate::primitives::send_attempt_result::SendAttemptResult;

/// A map of server-to-server notification failure reasons and counts that represent
/// the number of failures encountered during the performance test.
///
/// The keys are SendAttemptResult values describing the success or error the server encountered
/// as it attempted to send a notification.
///
/// [Failures](https://developer.apple.com/documentation/retentionmessaging/failures)
pub type Failures = HashMap<SendAttemptResult, i32>;