use std::future::Future;
use std::pin::pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use application::commands::RegisterUserCommand;
use application::errors::RegisterUserError;
use application::ports::{RepoFuture, UserRepository};
use application::use_cases::RegisterUserUseCase;
use application::validations::EmailNotTaken;
use domain::entities::User;
use domain::errors::DomainError;
use domain::value_objects::{Email, UserId};
use validations_rs::MinValidation;

/// Test double: the port implemented in memory.
#[derive(Default)]
struct InMemoryUsers {
    users: Mutex<Vec<User>>,
    last_id: AtomicU64,
}

impl UserRepository for InMemoryUsers {
    fn exists_by_email<'a>(&'a self, email: &'a Email) -> RepoFuture<'a, bool> {
        Box::pin(async move { Ok(self.users.lock().expect("lock").iter().any(|u| &u.email == email)) })
    }
    fn next_id(&self) -> RepoFuture<'_, UserId> {
        Box::pin(async { Ok(UserId(self.last_id.fetch_add(1, Ordering::SeqCst) + 1)) })
    }
    fn save<'a>(&'a self, user: &'a User) -> RepoFuture<'a, ()> {
        Box::pin(async move {
            self.users.lock().expect("lock").push(user.clone());
            Ok(())
        })
    }
}

/// Minimal executor for futures that never pend (no runtime needed in tests).
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(out) = future.as_mut().poll(&mut cx) {
            return out;
        }
    }
}

fn command(email: &str, age: u8, accepted_terms: bool) -> RegisterUserCommand {
    RegisterUserCommand { email: email.into(), age, accepted_terms }
}

#[test]
fn registers_then_rejects_duplicates_and_rule_violations() {
    let use_case = RegisterUserUseCase::new(Arc::new(InMemoryUsers::default()));

    let user = block_on(use_case.execute(command("Ana@Example.com", 30, true))).expect("valid");
    assert_eq!(user.email.as_str(), "ana@example.com");

    match block_on(use_case.execute(command("ana@example.com", 30, true))) {
        Err(RegisterUserError::BusinessRule(e)) => assert_eq!(e.code, EmailNotTaken::CODE),
        other => panic!("expected EMAIL_TAKEN, got {other:?}"),
    }
    match block_on(use_case.execute(command("teen@example.com", 16, true))) {
        Err(RegisterUserError::BusinessRule(e)) => assert_eq!(e.code, MinValidation::<u8>::CODE),
        other => panic!("expected VALUE_BELOW_MIN, got {other:?}"),
    }
    assert!(matches!(
        block_on(use_case.execute(command("no-arroba", 30, true))),
        Err(RegisterUserError::InvalidInput(DomainError::InvalidEmail(_)))
    ));
}

/// A repository whose queries fail, as if the database were down.
struct DownUsers;

impl UserRepository for DownUsers {
    fn exists_by_email<'a>(&'a self, _: &'a Email) -> RepoFuture<'a, bool> {
        Box::pin(async { Err(application::ports::RepositoryError("connection refused".into())) })
    }
    fn next_id(&self) -> RepoFuture<'_, UserId> {
        Box::pin(async { Ok(UserId(0)) })
    }
    fn save<'a>(&'a self, _: &'a User) -> RepoFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }
}

#[test]
fn a_database_failure_is_not_reported_as_a_broken_rule() {
    let use_case = RegisterUserUseCase::new(Arc::new(DownUsers));
    assert!(matches!(
        block_on(use_case.execute(command("ana@example.com", 30, true))),
        Err(RegisterUserError::RuleCheckFailed(_))
    ));
}

/// Web frameworks (axum, tonic) run handlers on a multi-threaded runtime, so the use-case
/// future must be `Send`. This test fails to compile if it ever stops being `Send`.
#[test]
fn the_use_case_future_is_send() {
    fn assert_send<T: Send>(_: &T) {}
    let use_case = RegisterUserUseCase::new(Arc::new(InMemoryUsers::default()));
    let future = use_case.execute(command("ana@example.com", 30, true));
    assert_send(&future);
}
