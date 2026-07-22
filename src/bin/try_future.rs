async fn announce(message: &str) {
    println!("{}", message);
}
#[tokio::main]
async fn main() {
    let bonnie = announce("Bonnie");
    let clyde = announce("Clyde");

    println!("Starring:");
    clyde.await;
    println!("and");
    bonnie.await;
}