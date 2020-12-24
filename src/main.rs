//#![feature(proc_macro_hygiene)]

// #![feature(decl_macro)]

// #[macro_use]
// extern crate rocket;
// extern crate rocket_contrib;

// use rocket::request::{Form, LenientForm};
// use rocket_contrib::serve::StaticFiles;
// use rocket::response::NamedFile;

// #[derive(FromForm)]
// struct Person {
//     name: String,
//     //age: Option<u8>
// }

// #[get("/hello?<name>")]
// fn collect(name: String ) -> String {
//     format!("name {}",name)
// }

// #[get("/hello?<name>")]
// fn collect(name: String ) -> Option<NamedFile> {

//     format!("name {}",name);
//     NamedFile::open("/static/").ok()
// }

// #[get("/")]
// fn index() -> Option<NamedFile> {
//     NamedFile::open("data_collecton/index.html").ok()
// }

// fn rocket() -> rocket::Rocket {
//     rocket::ignite()
//         .mount("/hello", StaticFiles::from("data_collection"))
//         .mount("/", routes![index, collect])
// }

// fn main() {
//     rocket().launch();
// }


// //**********crate Read  and write to text file **********/
// #![feature(decl_macro)]

// #[macro_use]
// extern crate rocket;
// extern crate rocket_contrib;

// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// #[get("/")]
// fn create() -> std::io::Result<()> {
//     //create a path for a file
//     let path = Path::new("new_fil.txt");
//     let display = path.display();

//     //print the path of a file
//     println!("{:?} {:?}", path, display);

//     //create a file at the the mentioned path
//     let mut file = File::create(path)?;

//     file.write_all(b"helo moto")?;
//     Ok(())
// }
// #[get("/read")]
// fn print() -> std::io::Result<String> {
//     let mut file = File::open("new_fil.txt")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }
// fn rocket()->rocket::Rocket{
//     rocket::ignite()
//         .mount("/", routes![create])
//         .mount("/read", routes![print])
// }

// fn main() {
//     rocket().launch();
// }

//**********crate json object at get request**********/
// #![feature(decl_macro)]
// #![feature(proc_macro_hygiene)]
	
// #[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;
	
// use std::sync::Mutex;
// use std::collections::HashMap;
	
// use rocket::State;
// use rocket_contrib::json::{Json, JsonValue};

// #[get("/message")]
// fn print_json()->JsonValue {
//     json!({
//         "name" : "osama",
//         "status" : "single"

//     })
// }

// #[catch(404)]
// fn not_found() -> JsonValue {
//     json!({
//         "status": "error",
//         "reason": "Resource was not found."
//     })
// }

// fn main() {
//     rocket().launch();
// }

// fn rocket()->rocket::Rocket{
//     rocket::ignite()
//         .mount("/", routes![print_json])
//         .register(catchers![not_found])
// }

// //**********crate json object at get request**********/
// #![feature(decl_macro)]
// #![feature(proc_macro_hygiene)]
	
// #[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;
	
// use std::sync::Mutex;
// use std::collections::HashMap;
	
// use rocket::State;
// use rocket_contrib::json::{Json, JsonValue};

// type ID = usize;

// //fro storing all the data
// type MessageMap = Mutex<HashMap<ID, String>>;

// #[derive(Serialize, Deserialize)]
// struct Message {
//     id: Option<ID>,
//     contents: String
// }

// #[post("/<id>", format = "json", data = "<message>")]
// fn post(id: ID, message: Json<Message>, map: State<'_, MessageMap>)-> JsonValue {
//     let mut hashmap = map.lock().unwrap();
//     if hashmap.contains_key(&id) {
//         json!({
//             "status": "error",
//             "reason": "id existes, tyr PUT"
//         })
//     }
//     else {
//         hashmap.insert(id, message.0.contents);
//         json!({
//             "status": "ok"
//            })
//     }
// }


// #[get("/<id>")]
// fn get(id: ID, map: State<'_, MessageMap>)-> Option<Json<Message>> {
//     let hashmap = map.lock().unwrap();
    
//     hashmap.get(&id).map(|contents| {
//         Json( Message {
//             id: Some(id),
//             contents: contents.clone()
//         })
//     }) 
// }

// #[catch(404)]
// fn not_found() -> JsonValue {
//     json!({
//         "status": "error",
//         "reason": "Resource was not found."
//     })
// }

// fn main() {
//     rocket().launch();
// }

// fn rocket()->rocket::Rocket{
//     rocket::ignite()
//         .mount("/", routes![get,post])
//         .register(catchers![not_found])
// }

//**********crate json object at get request**********/
#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]
	
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


#[get("/<id>")]
fn get(id: ID, map: State<'_, MessageMap>)-> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    
    hashmap.get(&id).map(|contents| {
        Json( Message {
            id: Some(id),
            contents: contents.clone()
        })
    }) 


fn main() {
    rocket().launch();
}

fn rocket()->rocket::Rocket{
    rocket::ignite()
        .mount("/", routes![get,post])
        .register(catchers![not_found])
}