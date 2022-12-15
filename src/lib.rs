use std::fs::{OpenOptions, read_dir};
use std::io::{Write, Read};
use std::path::PathBuf;
use std::{env, fs};

use anyhow::Result;

pub struct Pubsub {
    locale: PathBuf,
}

impl Pubsub {
    pub fn open(name: &str) -> Result<Pubsub> {
        let locale = env::temp_dir().join(name);
        let _ = std::fs::create_dir_all(&locale);

        Ok(Pubsub { locale })
    }

    pub fn publish(&self, message: &[u8], topic: &str) -> Result<()> {
        let topic_path = self.locale.join(topic);

        // If the topic file doesn't exist, create it
        if !topic_path.exists() {
            fs::File::create(&topic_path)?;
        }

        // Open the topic file for writing and append the message
        let mut topic_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&topic_path)?;

        topic_file.write_all(message)?;

        // Relay the message to each subscriber
        for entry in fs::read_dir(&self.locale)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            // If the file is a subscriber file (i.e., "<topic-name>-<subscription-token>")
            if file_name.starts_with(topic) && file_name.contains("-") {
                let mut sub_file = OpenOptions::new().write(true).append(true).open(&path)?;

                sub_file.write_all(message)?;
            }
        }

        Ok(())
    }

    pub fn receive(&self, sub_tok: &str) -> Result<Vec<u8>> {
        // Find the subscription file that matches the given subscription token
        let folder = &self.locale;
        let files = read_dir(folder).expect("Unable to read files in folder");
        let mut sub_file_name = "".to_string();
        
        for file in files {
            let path = file.expect("Unable to read file path").path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            if file_name.contains(sub_tok) {
                sub_file_name = file_name.clone();
                break;
            }
        }
        let sub_file_path = self.locale.join(sub_file_name);

        // Read the entire contents of the subscription file
        let mut sub_file = fs::File::open(&sub_file_path)?;
        let mut sub_file_contents = Vec::new();
        sub_file.read_to_end(&mut sub_file_contents)?;

        // Read the last message from the subscription file
        let last_message = sub_file_contents.split(|b| *b == b'\n').last().unwrap();

        // Truncate the subscription file to remove the last message
        let sub_file = fs::OpenOptions::new().write(true).open(&sub_file_path)?;
        sub_file.set_len((sub_file_contents.len() - last_message.len()) as u64)?;

        Ok(Vec::from(last_message))
    }

    pub fn subscribe(&self, topic: &str) -> Result<String> {
        // Generate a random UUID to use as the subscription token
        let sub_token = uuid::Uuid::new_v4().to_string();

        // Create the path for the subscription file
        let sub_file_path = self.locale.join(format!("{}-{}", topic, sub_token));

        // Clone the topic file to the subscription file
        std::fs::copy(self.locale.join(topic), &sub_file_path)?;

        Ok(sub_token)
    }
}
