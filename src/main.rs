/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
/*
MKProject Package Manager (mkpm)
By: MKProjects
Started: 2021-10-20
Developer: Mustafif Khan
License: GPLv2, MIT
Purpose: Package manager for various MKProject applications, such
as Minerva or Books.
*/
/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
#[allow(unused)]
pub mod options;
pub const DATA: &str = "data.json";

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(
    name = "mkpm",
    about = "Package manager for MKProject Applications/Books"
)]
enum CLI {
    #[structopt(about = "Get a specified book/application by it's id")]
    Get {
        #[structopt(short, long)]
        id: String,
    },
    #[structopt(about = "Search to get results of applications/books")]
    Search {
        #[structopt(short, long)]
        name: String,
    },
    #[structopt(about = "Lists out all available books/applications")]
    List {
        #[structopt(short = "M", long = "Max")]
        max: Option<u8>,
    },
    Update
    /*#[structopt(about = "Install a specified application by it's id")]
       Install {
           #[structopt(short, long)]
           id: String
       },
       */
}
#[tokio::main]
async fn main() {
    let cli = CLI::from_args();
    match cli {
        CLI::Get { id } => options::get(id).await.unwrap(),
        CLI::Search { name } => options::search(&name),
        /*
        CLI::Install {id}=> {
            install(id).await.unwrap();
        }
        */
        CLI::List {max} => options::list(max),
        CLI::Update => std::process::Command::new("cargo").arg("install").arg("mkpm").spawn().expect("Unable to update"),
        _ => {
            println!("Invalid command! Try mkpm --help");
        }
    }
}
