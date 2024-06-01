use zero2prod::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
// TODO start chapter 10.6.4.8 page 460 (pdf 479)
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read config.");
    let server = Application::build(configuration).await?;
    server.run_until_stopped().await?;
    Ok(())
}
