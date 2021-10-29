use clap::App;

mod pods;
mod err;

fn main() {
    let pod_app = App::new("pods")
        .about("Get pods")
        .arg("-n, --namespace=[namespace] 'List the pods'");

    let app = App::new("annyeong")
        .version("1.0")
        .author("Marc Intha")
        .about("Printing kubernetes pod")
        .subcommand(pod_app);

    let matches = app.get_matches();
    let res = match matches.subcommand() {
        Some(("pods", args)) => pods::trigger_list_pods(args),
        _ => Err(Box::new(err::AnnyeongError::CommandNotFound).into())
    };

    if let Err(err) = res {
        println!("{}", err)
    } 
}
