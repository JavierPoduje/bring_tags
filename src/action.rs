use std::process::{Command, Output, Stdio};

use crate::client::Client;

pub struct Action {
    pub client: Client,
}

impl Action {
    pub fn new(client: Client) -> Self {
        Action { client }
    }

    pub fn import_scenario(self, target_folder: String, dump_scenario: &str) -> Output {
        // 1. delete db on localhost
        println!("[INFO]: deleting scenario...");
        Command::new("mysql")
            .args([
                format!("--host={}", self.client.host),
                format!("--user={}", self.client.username),
                format!("--password={}", self.client.password),
                format!("--port={}", "4006"),
                format!("-e DROP DATABASE IF EXISTS {}", dump_scenario),
            ])
            .output()
            .expect("Couldn't drop db");

        // 2. create db
        println!("[INFO]: creating scenario...");
        Command::new("mysql")
            .args([
                format!("--host={}", self.client.host),
                format!("--user={}", self.client.username),
                format!("--password={}", self.client.password),
                format!("--port={}", "4006"),
                format!("-e CREATE DATABASE {}", dump_scenario),
            ])
            .output()
            .expect("Couldn't drop db");

        // 3. import db
        println!("[INFO]: importing scenario...");
        let cat = Command::new("cat")
            .args([format!("{}/{}.sql", target_folder.as_str(), dump_scenario)])
            .stdout(Stdio::piped())
            .spawn();

        Command::new("mysql")
            .args([
                format!("--host={}", self.client.host),
                format!("--user={}", self.client.username),
                format!("--password={}", self.client.password),
                format!("--port={}", "4006"),
                format!("--max_allowed_packet={}", "1024M"),
                format!("{}", dump_scenario),
            ])
            .stdin(cat.ok().unwrap().stdout.unwrap())
            .output()
            .expect("Couldn't import db")
    }

    pub fn dump_tags(self, ssh_alias: String) -> Output {
        Command::new("ssh")
        .args([
            ssh_alias,
            format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {} tags model_extensions", self.client.host, self.client.username, self.client.password, self.client.scenarios_db),
        ])
        .output()
        .expect("Couldn't get the dump...")
    }

    pub fn dump_scenario(self, ssh_alias: String, dump_scenario: &str) -> Output {
        Command::new("ssh")
        .args([
            ssh_alias,
            format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {}", self.client.host, self.client.username, self.client.password, dump_scenario),
        ])
        .output()
        .expect("Couldn't get the dump...")
    }
}
