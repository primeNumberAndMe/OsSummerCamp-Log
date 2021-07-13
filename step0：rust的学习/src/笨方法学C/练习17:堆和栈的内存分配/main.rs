use std::process;
use std::env;
use std::path::{PathBuf, Path};
use std::fs::{self};
use std::str::FromStr;



#[derive(Default)]
struct Address{
    id: i32,
    name: String,
    email: String,
}

impl std::str::FromStr for Address{
    type Err = &'static str;
    fn from_str(s: &str)->Result<Self, Self::Err>{
        //1 zed zed@e.com
        let v = s.split_whitespace().collect::<Vec<&str>>();
        let mut new_add = Address::default();
        let parsing_result = v[0].parse::<i32>();
        match parsing_result {
            Ok(id) => new_add.id = id,
            Err(_) => return Err("id parsing error"),
        }
        new_add.name = v[1].into();
        new_add.email = v[2].into();
        Ok(new_add)
    }
}

#[derive(Default)]
struct Database{
    rows: Vec<Address>,
}


#[derive(Default)]
struct Connection{
    path: PathBuf,
    db: Database,
}

impl Connection{
    fn database_load(&mut self){
        //assume the db file already exists
        let read_results = fs::read_to_string(self.path.as_path());
        if let Ok(contents) = read_results {
            if contents.is_empty() {
                return; //file is empty, there is nothing to mv;
            }else{
                //init the db from disk
                let item_vec: Vec<&str> = contents.split('\n').collect();
                for item in item_vec {
                    let new_addr = Address::from_str(item).unwrap();
                    self.db.rows.push(new_addr);
                }
            }
        }else{
            die("fs::read_to_string in database_load failed");
        }
    }

    fn database_open(filename: PathBuf)->Connection{
        let mut conn = Connection::default();
        conn.path = filename;
        conn.db = Database::default();
        conn
    }

    fn database_write(&self){
        //make all the addr instances a big string
        let mut all_addr_ins = String::new();
        for instance in self.db.rows.iter(){
            let single_ins = format!("{} {} {}", instance.id.to_string(), instance.name, instance.email);
            all_addr_ins.push_str(single_ins.as_str());
        }
        fs::write(self.path.as_path(), all_addr_ins).unwrap();
    }

    fn database_get(&self, id: i32){
        for item in self.db.rows.iter(){
            if item.id == id {
                item.address_print();
                break;
            }
        }
    }

    fn database_delete(&mut self, id: i32){
        self.db.rows.retain(|x| (*x).id != id);
    }

    fn database_list(&self){
        for item in self.db.rows.iter(){
            item.address_print();
        }
    }

    fn database_set(&mut self, id: i32, name: String, email: String){
        let new_addr = Address{id,name, email};
        self.db.rows.push(new_addr);
    }
}

fn die(err: &str){
    //print some err info
    eprintln!("{}", err);
    //then exit
    process::exit(1);
}

impl Address{
    fn address_print(&self){
        println!("{} {} {}", self.id, self.name, self.email);
    }
}

fn main(){
    let arg_v = env::args().collect::<Vec<String>>();
    let arg_c = arg_v.len();
    //supply at least 2 args
    if arg_c < 3 {
        die("USAGE: ex17 <dbfile> <action> [action params]");
    }

    let filename = arg_v[1].clone();

    let mut action  = 'u';
    if let Some(ref_to_u8) = arg_v[2].as_bytes().first() {
        action = (*ref_to_u8) as char;
    }

    let mut conn = Connection::database_open(Path::new(filename.as_str()).to_path_buf());
    let mut id = -1;

    if arg_c > 3 {
        id = arg_v[3].parse().unwrap();
    }

    // if id > conn.db.rows.len(){
    //     die("There's not that many records.");
    // }

    match action {
        'c' => {
            conn.database_write();
        },
        'g' => {
            if arg_c != 4 {
                die("Need an id to get");
            }else{
                conn.database_load();
                conn.database_get(id);
            }
        },
        's' => {
            if arg_c != 6 {
                die("Need id, name, email to set");
            }else{
                conn.database_set(id, arg_v[4].clone(), arg_v[5].clone());
                conn.database_write();
            }
        },
        'd' => {
            if arg_c != 4 {
                die("Need id to delete");
            }else{
                conn.database_delete(id);
                conn.database_write();
            }
        },
        'l' => {
            conn.database_load();
            conn.database_list();
        },
        _ => die("Invalid action, only: c=create, g=get, s=set, d=del, l=list"),
    }
}
