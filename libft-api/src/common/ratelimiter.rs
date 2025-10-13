use reqwest::header::HeaderMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct HeaderMetaData {
    pub ratelimiter: RateLimiter,
    pub total_page: Arc<Mutex<u64>>,
}

impl HeaderMetaData {
    pub fn new(ratelimiter: RateLimiter) -> Self {
        Self {
            ratelimiter,
            total_page: Arc::new(Mutex::new(0)),
        }
    }

    pub fn update_from_headers(&self, headers: &HeaderMap) {
        let parse_header = |name: &str| -> Option<u64> {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
        };

        if let Some(total) = parse_header("x-total") {
            *self.total_page.lock().unwrap() = total;
        }
        if let Some(remaining) = parse_header("x-secondly-ratelimit-remaining") {
            *self
                .ratelimiter
                .secondly_ratelimit_remaining
                .lock()
                .unwrap() = remaining;
        }

        if let Some(remaining) = parse_header("x-hourly-ratelimit-remaining") {
            *self.ratelimiter.hourly_ratelimit_remaining.lock().unwrap() = remaining;
        }

        if let Some(retry_seconds) = parse_header("retry-after") {
            let wait_duration = Duration::from_secs(retry_seconds);
            let new_retry_time = SystemTime::now() + wait_duration;
            *self.ratelimiter.retry_after.lock().unwrap() = new_retry_time;
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub secondly_ratelimit_limit: u64,
    hourly_ratelimit_limit: u64,
    secondly_ratelimit_remaining: Arc<Mutex<u64>>,
    hourly_ratelimit_remaining: Arc<Mutex<u64>>,

    // 429 응답의 `retry-after` 헤더를 처리하기 위한 필드
    retry_after: Arc<Mutex<SystemTime>>,
    // 초당 요청 제한 윈도우가 언제 리셋되는지 추적하기 위한 필드 (추가됨)
    secondly_window_reset: Arc<Mutex<SystemTime>>,
    hourly_window_reset: Arc<Mutex<SystemTime>>,
}

impl RateLimiter {
    pub fn new(per_second_limit: u64, hourly_limit: u64) -> Self {
        let now = SystemTime::now();
        Self {
            secondly_ratelimit_limit: per_second_limit,
            hourly_ratelimit_limit: hourly_limit,
            secondly_ratelimit_remaining: Arc::new(Mutex::new(per_second_limit)),
            hourly_ratelimit_remaining: Arc::new(Mutex::new(hourly_limit)),
            retry_after: Arc::new(Mutex::new(now)),
            secondly_window_reset: Arc::new(Mutex::new(now + Duration::from_secs(1))),
            hourly_window_reset: Arc::new(Mutex::new(now + Duration::from_secs(3600))),
        }
    }

    /// API 요청을 보내기 전에 호출하여 ratelimit을 준수하도록 대기합니다.
    pub async fn acquire(&self) {
        loop {
            let now = SystemTime::now();

            // 1. 'retry-after'에 의한 강제 대기 확인
            // 외부에서 429 응답을 받았을 때 이 값을 업데이트했다고 가정합니다.
            let retry_after_time = *self.retry_after.lock().unwrap();
            if now < retry_after_time {
                let wait_duration = retry_after_time.duration_since(now).unwrap_or_default();
                sleep(wait_duration).await;
                continue; // 대기 후 루프의 처음부터 다시 확인
            }

            // Mutex Guard가 await 지점까지 살아남지 않도록 범위를 제한합니다.
            let wait_duration = {
                // 2. 초당 요청 제한 확인 (Fixed Window)
                let mut remaining_guard_sec = self.secondly_ratelimit_remaining.lock().unwrap();
                let mut remaining_guard_hour = self.hourly_ratelimit_remaining.lock().unwrap();
                let mut reset_guard_sec = self.secondly_window_reset.lock().unwrap();
                let mut reset_guard_hour = self.hourly_window_reset.lock().unwrap();

                if now >= *reset_guard_sec {
                    *remaining_guard_sec = self.secondly_ratelimit_limit;
                    *reset_guard_sec = now + Duration::from_secs(1);
                }

                if now >= *reset_guard_hour {
                    *remaining_guard_hour = self.hourly_ratelimit_limit;
                    *reset_guard_hour = now + Duration::from_secs(3600);
                }

                if *remaining_guard_hour > 0 {
                    *remaining_guard_hour -= 1;
                    if *remaining_guard_sec > 0 {
                        *remaining_guard_sec -= 1;
                        break;
                    } else {
                        reset_guard_sec.duration_since(now).unwrap_or_default()
                    }
                } else {
                    reset_guard_hour.duration_since(now).unwrap_or_default()
                }
            };

            // wait_duration이 0보다 크면 대기합니다.
            if !wait_duration.is_zero() {
                sleep(wait_duration).await;
            }
            // 대기 후, 다른 스레드가 먼저 토큰을 가져갔을 수 있으므로 loop를 다시 돕니다.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// 테스트 1: 제한 횟수 내에서는 대기 없이 바로 통과하는지 확인
    #[tokio::test]
    async fn test_can_acquire_within_limit() {
        let limiter = RateLimiter::new(5, 100); // 초당 5회
        let start = Instant::now();

        for _ in 0..5 {
            limiter.acquire().await;
        }

        let elapsed = start.elapsed();
        println!("Elapsed time for 5 acquires: {:?}", elapsed);

        // 5번의 호출은 sleep 없이 즉시 처리되어야 하므로 매우 짧은 시간 안에 끝나야 합니다.
        assert!(elapsed < Duration::from_millis(100));
    }

    /// 테스트 2: 초당 제한을 초과하면 1초 이상 대기하는지 확인
    #[tokio::test]
    async fn test_waits_when_per_second_limit_exceeded() {
        let limiter = RateLimiter::new(3, 100); // 초당 3회
        let start = Instant::now();

        // 4번 호출 -> 3번은 즉시, 1번은 1초 뒤에 실행되어야 함
        for _ in 0..4 {
            limiter.acquire().await;
        }

        let elapsed = start.elapsed();
        println!(
            "Elapsed time for 4 acquires with 3/sec limit: {:?}",
            elapsed
        );

        // 총 소요 시간은 최소 1초 이상이어야 합니다.
        assert!(elapsed >= Duration::from_secs(1));
        // 너무 오래 기다리는 것도 아니어야 합니다. (네트워크 지연 등 감안)
        assert!(elapsed < Duration::from_secs(2));
    }

    /// 테스트 3: 1초가 지나면 제한이 초기화되는지 확인
    #[tokio::test]
    async fn test_limit_resets_after_one_second() {
        let limiter = RateLimiter::new(2, 100); // 초당 2회

        // 1. 먼저 제한을 모두 소진
        limiter.acquire().await;
        limiter.acquire().await;

        // 2. 1초보다 길게 대기하여 윈도우를 리셋
        sleep(Duration::from_millis(1100)).await;

        // 3. 리셋된 후에는 다시 즉시 통과해야 함
        let start = Instant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        let elapsed = start.elapsed();

        println!("Elapsed time for 2 acquires after reset: {:?}", elapsed);

        // 리셋된 후의 호출은 다시 빨라야 합니다.
        assert!(elapsed < Duration::from_millis(100));
    }

    /// 테스트 4: 여러 태스크에서 동시에 접근해도 안전하게 동작하는지 확인
    #[tokio::test]
    async fn test_concurrent_acquires_are_safe() {
        // Arc를 통해 여러 태스크가 RateLimiter를 공유
        let limiter = Arc::new(RateLimiter::new(8, 100)); // 초당 5회
        let mut tasks = vec![];

        let start = Instant::now();

        // 10개의 태스크를 생성하여 동시에 acquire 호출
        for i in 0..64 {
            let limiter_clone = Arc::clone(&limiter);
            tasks.push(tokio::spawn(async move {
                println!("Task {} acquiring...", i);
                limiter_clone.acquire().await;
                println!("Task {} acquired.", i);
            }));
        }

        // 모든 태스크가 끝날 때까지 대기
        futures::future::join_all(tasks).await;

        let elapsed = start.elapsed();
        println!(
            "Elapsed time for 64 concurrent acquires with 8/sec limit: {:?}",
            elapsed
        );

        assert!(elapsed >= Duration::from_secs(7));
        assert!(elapsed < Duration::from_secs(8));
    }

    use reqwest::header::HeaderValue;
    /// 테스트 5: 정상 응답(200 OK) 헤더를 받았을 때 상태가 올바르게 업데이트되는지 확인
    #[test]
    fn test_update_from_successful_response() {
        let meta = HeaderMetaData::new(RateLimiter::new(2, 1200));

        // 시뮬레이션할 헤더 생성
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-secondly-ratelimit-remaining",
            HeaderValue::from_static("0"),
        );
        headers.insert(
            "x-hourly-ratelimit-remaining",
            HeaderValue::from_static("1194"),
        );

        // 상태 업데이트
        meta.update_from_headers(&headers);

        // 상태 검증
        let secondly_remaining = *meta
            .ratelimiter
            .secondly_ratelimit_remaining
            .lock()
            .unwrap();
        let hourly_remaining = *meta.ratelimiter.hourly_ratelimit_remaining.lock().unwrap();

        assert_eq!(
            secondly_remaining, 0,
            "Secondly remaining should be updated to 0"
        );
        assert_eq!(
            hourly_remaining, 1194,
            "Hourly remaining should be updated to 1194"
        );
    }

    /// 테스트 6: 속도 제한 응답(429) 헤더를 받았을 때 retry_after가 올바르게 설정되는지 확인
    #[test]
    fn test_update_from_ratelimited_response() {
        let meta = HeaderMetaData::new(RateLimiter::new(2, 1200));
        let before_update = SystemTime::now();

        // 429 응답 시뮬레이션 헤더
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_static("1"));

        // 상태 업데이트
        meta.update_from_headers(&headers);

        // 상태 검증
        let retry_time = *meta.ratelimiter.retry_after.lock().unwrap();

        // retry_time이 업데이트 이전 시간보다 미래인지 확인
        assert!(
            retry_time > before_update,
            "retry_after time should be in the future"
        );

        // 대기 시간이 약 1초인지 확인 (실행 시간 오차 감안)
        let wait_duration = retry_time.duration_since(before_update).unwrap();
        assert!(
            wait_duration >= Duration::from_millis(900)
                && wait_duration <= Duration::from_millis(1100),
            "The wait duration should be approximately 1 second. Actual: {:?}",
            wait_duration
        );
    }
}
