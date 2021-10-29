use clap::App;

mod pods;
mod err;

fn main() {
    let pod_app = App::new("pods")
        .about("Get pods")
        .arg("list 'List the pods'")
        .short_flag('l');

    let app = App::new("annyeong")
        .version("1.0")
        .author("Marc Intha")
        .about("Printing kubernetes pod")
        .subcommand(pod_app);

    let matches = app.get_matches();
    let res = match matches.subcommand() {
        Some(("list", args)) => pods::trigger_list_pods(),
        _ => Err(Box::new(err::AnnyeongError::CommandNotFound).into())
    };

    if let Err(err) = res {
        println!("{}", err)
    } 
}
