// Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bellande_probability", about = "Bellande Probability Tool")]
struct Opt {
    #[structopt(long, help = "First coordinate as JSON-formatted list")]
    node0: String,
    
    #[structopt(long, help = "Second coordinate as JSON-formatted list")]
    node1: String,
    
    #[structopt(long, help = "Probability threshold value")]
    threshold: f64,
    
    #[structopt(long, help = "Number of dimensions")]
    dimensions: i32,
    
    #[structopt(long, help = "Use full authentication")]
    full_auth: bool,
}

async fn make_bellande_probability_request(
    node0: Value,
    node1: Value,
    threshold: f64,
    dimensions: i32,
    full_auth: bool,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Probability";
    
    let endpoint = if full_auth {
        format!("{}/bellande_probability_full_auth", base_url)
    } else {
        format!("{}/bellande_probability", base_url)
    };

    let auth = if full_auth {
        json!({
            "full_authorization_key": "bellande_web_api_full_auth"
        })
    } else {
        json!({
            "authorization_key": "bellande_web_api_opensource"
        })
    };

    let payload = json!({
        "node0": node0,
        "node1": node1,
        "threshold": threshold,
        "dimensions": dimensions,
        "auth": auth
    });

    let response = client
        .post(&endpoint)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let node0: Value = serde_json::from_str(&opt.node0)
        .map_err(|e| format!("Error parsing node0: {}", e))?;
    let node1: Value = serde_json::from_str(&opt.node1)
        .map_err(|e| format!("Error parsing node1: {}", e))?;

    match make_bellande_probability_request(
        node0,
        node1,
        opt.threshold,
        opt.dimensions,
        opt.full_auth
    ).await {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
