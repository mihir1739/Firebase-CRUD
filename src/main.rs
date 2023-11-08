use firebase_rs::*;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String
}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let connection = env::var("DATABASE_URL").expect("You've not set the DATABASE_URL");
    let user = User {
        name: "Mihir Kumar Singh".to_string(),
        age: 22,
        email: "mihir39nr@gmail.com".to_string()
    };
    let firebase = Firebase::new(&connection).unwrap();
    let response = set_user(&firebase, &user).await;
    let mut user = get_user(&firebase, &response.name).await;
    let users = get_users(&firebase).await;
    println!("{:?}",users);

    user.email = "newmail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name,&user).await;
    println!("{:?}",updated_user);
    delete_user(&firebase, &response.name).await;
    println!("{}","User Records deleted");
}

async fn set_user(firebase_client : &Firebase,user: &User) -> Response{
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client : &Firebase) -> HashMap<String, User>{
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String,User>>().await;
    println!("{:?}",users);
    return users.unwrap();
}

async fn get_user(firebase_client : &Firebase,id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client : &Firebase, id: &String, user: &User) -> User{
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client : &Firebase,id: &String){
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// convert a string to response 
fn string_to_response(s:&str) -> Response{
    serde_json::from_str(s).unwrap()
}

fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}