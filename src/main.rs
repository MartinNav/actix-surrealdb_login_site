use actix_web::{self, HttpServer, Responder, HttpResponse, App};
use cookie::Cookie;
use db::User;
use lazy_static::lazy_static;
use serde;
use surreal_simple_client::SurrealClient;
use std::sync::Mutex;
mod load;
mod db;
//These global variables exist just because loading the files each time the request comes is to time consuming
//file content will be loaded only on the first page load
lazy_static!(
    #[derive(Debug)]
    static ref INDEX_HTML:String = load::safe_read_file("WWW/index.html".to_string());
    #[derive(Debug)]
    static ref LOGIN_HTML:String = load::safe_read_file("WWW/login.html".to_string());
    #[derive(Debug)]
    static ref REGISTER_HTML:String = load::safe_read_file("WWW/register.html".to_string());


    //static ref DB:Mutex<SurrealClient>= Mutex::new(tokio::runtime::Runtime::new().unwrap().block_on(async{ db::create_db().await}));
);



#[actix_web::get("/")]
async fn index_page()-> impl Responder{

HttpResponse::Ok().body(format!("{}",INDEX_HTML.as_str()))
}

#[actix_web::get("/login")]
async fn login()-> impl Responder{
    HttpResponse::Ok().body(format!("{}",LOGIN_HTML.as_str()))
}

async fn login_activity(param:actix_web::web::Form<db::User>)->HttpResponse{
    let mut db = db::create_db().await;
    let user_login =db::User{
        uname:param.uname.clone(),
        psw:param.psw.clone(),
        token:param.token.clone()
    };
    let db_res:Option<db::User> = db.find_one("SELECT * FROM user WHERE uname = $uname;".to_owned(),
     serde_json::json!({"uname" : user_login.uname}))
     .await
     .unwrap();     

    match db_res {
        Some(u_db)=>{
            if u_db.uname == user_login.uname && u_db.psw == user_login.psw{
                //TODO: Unique cookies for login
                // for now there will be just default login cookie and that is not secure
                return HttpResponse::SeeOther().cookie(Cookie::build("acces", "granted").expires(None).finish()).insert_header(("Location","/restricted")).finish();
            }
        }
        _=>{}
    }

    HttpResponse::SeeOther().insert_header(("Location","/login")).finish()
}

#[actix_web::get("/register")]
async fn register()-> impl Responder{
    HttpResponse::Ok().body(format!("{}",REGISTER_HTML.as_str()))
}

async fn register_activity(param: actix_web::web::Form<db::User>)->HttpResponse{
    let (mut user,mut password) = (param.uname.clone(),param.psw.clone());
    let mut db = db::create_db().await;


    db.send_query("CREATE user SET uname = $uname, psw = $psw;".to_owned(), //Even when it displys nosense it works
    serde_json::json!(
        {"uname":user,
        "psw":password
        }
    ))
    .await
    .unwrap();
    
    HttpResponse::SeeOther().insert_header(("Location","/login")).finish()
}

async fn not_found()->HttpResponse{
    HttpResponse::NotFound().body(load::safe_read_file("WWW/404.html".to_string()))
}

#[actix_web::get("/restricted")]
async fn restricted(req: actix_web::HttpRequest)->HttpResponse{
    let acces_cookie = req.cookie("acces");
    match acces_cookie {
        Some(ac)=>{
            if ac.value() == "granted"{
                return HttpResponse::Accepted().body(load::safe_read_file("WWW/restricted.html".to_string()));
            }
        }
        _=>{}
    }
    HttpResponse::SeeOther().insert_header(("Location","/login")).finish()
    
}



///This project is not secured nor scalable,
///because it does not clean its own db connections and it also does use the same authentication cookie withou expiration for all users



#[actix_web::main]
async fn main()->std::io::Result<()> {
    //println!("Hello, world!");

    HttpServer::new(||{
        App::new()
        .service(index_page)
        .service(login)
        .service(register)
        .route("/regact", actix_web::web::post().to(register_activity))
        .route("/logact", actix_web::web::post().to(login_activity))
        .service(restricted)
        .default_service(actix_web::web::route().to(not_found))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await

}
