use std::{env, io::{self, Write}, process, time::Duration};

extern crate paho_mqtt as mqtt;

const DFLT_BROKER: &str = "tcp://localhost:1883";
const DFLT_CLIENT: &str = "rust_publish";

fn main() {
    // Parse command line arguments
    let host = env::args().nth(1).unwrap_or_else(|| DFLT_BROKER.to_string());

    // Set up MQTT client
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host.clone())
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    let mut cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    // Connect to the broker
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // Interactive loop for publishing messages
    loop {
        let content = get_user_input("Enter your message (type 'exit' to quit): ");
        
        // Check if the user wants to exit
        if content.to_lowercase() == "exit" {
            break;
        }

        let topic = get_user_input("Enter the topic: ");

        // Create and publish the message
        let qos = 1; // You can customize the QoS level as needed
        let msg = mqtt::Message::new(topic, content.clone(), qos);
        let tok = cli.publish(msg);

        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);
            break;
        }
    }

    // Disconnect from the broker
    let tok = cli.disconnect(None);
    println!("Disconnected from the broker");
    tok.unwrap();
}

// Helper function to get user input
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    
    user_input.trim().to_string()
}
