use clap;
use clap::{command, Arg, ArgAction, ArgGroup, ArgMatches};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use std::env;

struct Things{
    exists : bool,
    number : usize,
    currentchat : String,
    date : String,
    homepath : String,

}
#[derive(Deserialize)]
struct ResponseData{
    response : String,
}

#[derive(Serialize)]
struct RequestData{
    request : String,

}

impl Things {
    //this is complete checks if the directory exists
    fn CheckIfAiDirExists(&mut self){
        let  home = env::var("HOME").unwrap();
        self.homepath = home + "/Aichatserver/";
        let path = Path::new(self.homepath.as_str());
        if path.exists(){
            self.exists = true
        }else {
            fs::create_dir(&self.homepath).unwrap()
        }
    }
    //this is complete how many files there is
    fn CheckIfChatFileExists(&mut self)->usize{
        let total_entries = 0;
        for entry in fs::read_dir(&self.homepath).unwrap(){
            let total_entries = total_entries + 1;
        }
        total_entries
    }

    //this is complete deletes the files
    //used this one
    fn delete_files(&mut self,match_results: ArgMatches){
        let delete_all = match_results.get_one::<bool>("delete").unwrap().clone();
        if delete_all{
            for entry in fs::read_dir(&self.homepath).unwrap(){
                let path = entry.unwrap().path();
                fs::remove_file(path).unwrap();
            }
        }else{
            let delete_certain = match_results.get_one::<usize>("deletecertain").unwrap().clone();
            let mut times:Vec<(SystemTime,PathBuf)> = vec![];
            for entry in fs::read_dir(&self.homepath).unwrap(){
                let path = entry.unwrap().path();
                times.push((fs::metadata(path.clone()).unwrap().modified().unwrap(),path));
            }
            times.sort();
            for i in 0..delete_certain{
                fs::remove_file(times[i].1.clone()).unwrap();
            }
        }
    }

    //this is complete lists all the files in the directory
    //used this one
    fn listallchats(&mut self,match_results: ArgMatches){
        if match_results.get_one::<bool>("listall").unwrap().clone(){
            for entry in fs::read_dir(&self.homepath).unwrap(){
                let path = entry.unwrap().path();
                println!("{}",path.display());

            }
        }
    }

    //this is complete creates a new chat file
    //used this one
    fn newchat(&mut self, match_results: ArgMatches){
        let name = match_results.get_one::<String>("newchat").unwrap().clone();
        let path =
            fs::File::create(String::from(&self.homepath) + &name).unwrap();
        self.currentchat = name;
        println!("new chat created");

    }
    fn currentchatinfo(&mut self){
        println!("{}",self.currentchat);
    }



    //done this one to send request
    //used this one
    async fn digest(match_results:&ArgMatches,function: &str)->Result<String, Box<dyn std::error::Error>>{
        let prompt:String = match_results.get_one::<String>(function).unwrap().clone();
        let client = reqwest::Client::new();
        let timeout = Duration::from_secs(15);
        let url = format!("http://10.0.0.1:8000/{}?prompt={}", function, prompt);
        let resp: ResponseData = client
            .post(url)
            .timeout(timeout)
            .send()
            .await?
            .json::<ResponseData>()
            .await?;
        println!("{}",resp.response);
        Ok(resp.response)

    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let match_results: ArgMatches = command!()
        .arg(Arg::new("generate").short('p').long("prompt").help("this is for the sending a prompt to the AI").action(ArgAction::Set).num_args(1..))//done this one
        .arg(Arg::new("newchat").short('n').long("newchat").help("to start a new chat.").action(ArgAction::Set).num_args(1..))//done this one
        .arg(Arg::new("history").short('s').long("history").help("to display all the chat history history").action(ArgAction::SetTrue))
        .arg(Arg::new("delete").short('d').long("delete").help("to delete all the chat history").action(ArgAction::Set).num_args(1..))//done this one
        .arg(Arg::new("deletecertain").short('a').long("deletecertain").help("to delete a number of chat history").action(ArgAction::Set).num_args(1..))// done this one
        .arg(Arg::new("listall").short('l').long("listall").help("to list all the chat history").action(ArgAction::SetTrue))//done this one
        .arg(Arg::new("digest").short('c').long("command").help("to make it generate a command").action(ArgAction::Set).num_args(1..))// done this one
        .group(ArgGroup::new("mode").args(&["digest","generate"]).multiple(false).required(false))
        .group(ArgGroup::new("Delete").args(&["delete","deletecertain","newchat"]).multiple(false).required(false))
        .get_matches();

    let mut things = Things{
        exists: false,
        number: 0,
        currentchat: "".to_string(),
        date: "".to_string(),
        homepath: "".to_string(),
    };

    things.CheckIfAiDirExists();
    let gene = match_results.get_one::<String>("generate");
    let dig = match_results.get_one::<String>("digest");

    match (gene,dig) {
        ( Some(gene) , None ) => {
            Things::digest(&match_results,"generate").await?;
        }
        ( None , Some(dig) ) => {
            Things::digest(&match_results,"digest").await?;
        }
        _ => {
        }
    }

    let nc = match_results.get_one::<String>("newchat");
    let dc = match_results.get_one::<String>("deletecertain");
    let dl = match_results.get_one::<String>("delete");
    match (nc,dc,dl) {
        (Some(nc), None, None) => {
            things.newchat(match_results.clone());
            println!("new chat function entered");
        }
        (None, Some(dc), None ) => {
            things.delete_files(match_results.clone());
            println!("delete certain function entered");
        }
        (None,None,Some(dl)) => {
            things.delete_files(match_results.clone());
            println!("delete all function entered");
        }
        _ =>{
            println!("no function selected");
        }
    }
    let mut list = Some(match_results.get_one::<bool>("listall"));
    if list != None{
        things.listallchats(match_results.clone());
    }
    let mut history = Some(match_results.get_one::<bool>("history").unwrap().clone());
    if history != None{
        things.listallchats(match_results.clone());
    }

    Ok(())
}
