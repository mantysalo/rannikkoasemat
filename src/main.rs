use rannikkoasemat;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rannikkoasemat::rocket().launch().await;
    Ok(())
}
