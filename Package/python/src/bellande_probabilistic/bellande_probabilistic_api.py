# Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

#!/usr/bin/env python3
import requests
import argparse
import json
import sys

def make_bellande_probability_distribution_request(mu_func, sigma_func, x, dimensions, full_auth=False):
    base_url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Probability"
    
    endpoint = f"{base_url}/bellande_probability_full_auth" if full_auth else \
               f"{base_url}/bellande_probability"
    
    # Convert string input to list if it's a string
    if isinstance(x, str):
        x = json.loads(x)
    
    auth = {
        "full_authorization_key": "bellande_web_api_full_auth"
    } if full_auth else {
        "authorization_key": "bellande_web_api_opensource"
    }
    
    payload = {
        "mu_func": mu_func,
        "sigma_func": sigma_func,
        "x": x,
        "dimensions": dimensions,
        "auth": auth
    }
    
    headers = {
        'accept': 'application/json',
        'Content-Type': 'application/json'
    }
    
    try:
        response = requests.post(endpoint, json=payload, headers=headers)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        print(f"Error making request: {e}", file=sys.stderr)
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(description="Run Bellande Distribution API")
    parser.add_argument("--mu-func", required=True, help="mu function as string")
    parser.add_argument("--sigma-func", required=True, help="sigma function as string")
    parser.add_argument("--x", required=True, help="Input vector as JSON-formatted list")
    parser.add_argument("--dimensions", type=int, required=True, help="Number of dimensions")
    parser.add_argument("--full-auth", action="store_true", help="Use full authentication")
    
    args = parser.parse_args()
    
    try:
        result = make_bellande_probability_distribution_request(
            args.mu_func,
            args.sigma_func,
            args.x,
            args.dimensions,
            args.full_auth
        )
        
        print(json.dumps(result, indent=2))
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON format in input - {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
