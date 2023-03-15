use tokio::runtime::Runtime;


#[derive(serde::Deserialize,Debug)]
pub struct User{
    pub uname:String,
    pub psw:String,
    pub token:Option<String>,
}



///# Create DB
/// this function creates connection to surraelDB
///## Terminal
/// In terminal before you run this application you have to write
/// ```
/// surreal start --log trace --user authsite --pass pa55w0rd memory
/// ``` 
/// The data will be saved only in your **RAM** not disc/SSD
/// 
pub async fn create_db()->surreal_simple_client::SurrealClient{
//    let rt = Runtime::new().expect("Unable to start tokio runtime");

    
    let mut client = 
        surreal_simple_client::SurrealClient::new("ws://localhost:8000/rpc")    
    .await
    .expect("Connection to DB can not be created");


        client.signin("authsite", "pa55w0rd")
    .await
    .expect("Unable to authenticate");

        client.use_namespace("site", "auth")
    .await
    .expect("Unable to setup namespase & DB");
    
    client
}