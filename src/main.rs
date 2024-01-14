use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
struct Args
{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands
{
    Play
    {
    },
}

fn render() 
{
    for y in 1..=3 {
        for x in 1..=3 {
            print!("{} {} ", x * y, if x == 3 { "" } else { "-" });
        }
        println!("");
    }
}

fn play()
{

}


fn main() 
{
    let args = Args::parse();

    match &args.command 
    {
        Some(Commands::Play {}) =>
        {
            render();
            play();
        }
        None => {}
    }
}
