use std::time::{SystemTime, Duration};

use rocket::{routes, fs::FileServer, serde::{json::{Json, serde_json::json, Value, self}, Serialize}, get, State, post, tokio::sync::RwLock};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    
    let _rocket = rocket::build()
        .manage(RwLock::new(Vec::<PlayerNotice>::new()))
        .mount("/", FileServer::from("www/"))
        .mount("/", routes![get_list, log_player])
        .launch()
        .await?;
    Ok(())
}

#[get("/api/getlist")]
async fn get_list(state: &State<RwLock<Vec<PlayerNotice>>>) -> Json<Value> {

    let read = state.read().await;

    let values = match json::to_value(read.as_slice()) {
        Ok(x) => x,
        Err(_) => json!(""), // return "null"
    };

    Json(values)
}

#[post("/api/logplayer", data="<input>")]
async fn log_player(input: &str, state: &State<RwLock<Vec<PlayerNotice>>>) {

    let mut read = state.write().await; 

    // see if the player has been seen before
    let need_to_add = read
        .iter()
        .all(|f| f.name != input);
    

    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO);  
    
    // if the player hasn't been see before, add them to the list
    if need_to_add {
        read.push(PlayerNotice {
            timestamp: time,
            name: input.to_owned()
        });
    // otherwise just modify their last see time
    } else {
        read
            .iter_mut()
            .filter(|f| f.name == input)
            .for_each(|f| {
                f.timestamp = time;
            });
    }

}

#[derive(Serialize)]
struct PlayerNotice {
    timestamp: Duration,
    name: String
}