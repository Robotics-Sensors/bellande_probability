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
#[structopt(
    name = "bellande_probability",
    about = "Bellande Distribution Probability Tool"
)]
struct Opt {
    #[structopt(long, help = "mu function as string")]
    mu_func: String,

    #[structopt(long, help = "sigma function as string")]
    sigma_func: String,

    #[structopt(long, help = "Input vector as JSON-formatted list")]
    x: String,

    #[structopt(long, help = "Number of dimensions")]
    dimensions: i32,

    #[structopt(long, help = "Use full authentication")]
    full_auth: bool,
}

pub async fn make_bellande_probability_request(
    mu_func: String,
    sigma_func: String,
    x: Value,
    dimensions: i32,
    full_auth: bool,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url =
        "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Probability";

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
        "mu_func": mu_func,
        "sigma_func": sigma_func,
        "x": x,
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

pub async fn make_bellande_probability_request_local(
    url: &str,
    mu_func: String,
    sigma_func: String,
    x: Value,
    dimensions: i32,
    full_auth: bool,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url = url;

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
        "mu_func": mu_func,
        "sigma_func": sigma_func,
        "x": x,
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
