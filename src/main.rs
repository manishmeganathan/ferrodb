mod netswarm;
mod netbehaviour;

#[async_std::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Setup the p2p swarm network
    netswarm::setup_swarm();
}