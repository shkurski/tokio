#[tokio::test]
async fn join_handle() {
    let result = tokio::spawn(async { 1 + 1 }).await;
    assert_eq!(2, result.unwrap());
}

#[tokio::test]
#[should_panic]
async fn join_handle_panic() {
    tokio::spawn(async {
        panic!("should panic");
    })
    .await
    .unwrap();
}
