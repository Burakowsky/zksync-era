use zksync_dal::ConnectionPool;

use crate::{CircuitBreaker, CircuitBreakerError};

// Struct for monitoring replication lag.
#[derive(Debug)]
pub struct ReplicationLagChecker {
    pub pool: ConnectionPool,
    pub replication_lag_limit_sec: Option<u32>,
}

// Implementing the CircuitBreaker trait for ReplicationLagChecker.
#[async_trait::async_trait]
impl CircuitBreaker for ReplicationLagChecker {
    // Implementing the check method defined in the CircuitBreaker trait.
    async fn check(&self) -> Result<(), CircuitBreakerError> {
        // Get the replication lag from the system and perform checks.
        let lag = self
            .pool
            .access_storage()
            .await
            .unwrap()
            .system_dal()
            .get_replication_lag_sec()
            .await;

        // Record replication lag in metrics.
        metrics::gauge!("circuit_breaker.replication_lag", lag as f64);

        // Check if the replication lag exceeds the defined limit.
        match self.replication_lag_limit_sec {
            Some(replication_lag_limit_sec) if lag > replication_lag_limit_sec => {
                // Return an error if the replication lag exceeds the limit.
                Err(CircuitBreakerError::ReplicationLag(lag, replication_lag_limit_sec))
            }
            _ => Ok(()),
        }
    }
}

