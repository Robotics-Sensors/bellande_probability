# 📦 Bellande Probabilistic (Infinite Dimentions)

## 🧙 Organization Website
- [![Organization Website](https://img.shields.io/badge/Explore%20Our-Website-0099cc?style=for-the-badge)](https://robotics-sensors.github.io)


## 🧙 Organization Github
- [![Organization Github ](https://img.shields.io/badge/Explore%20Our-Github-0099cc?style=for-the-badge)](https://github.com/Robotics-Sensors)

# Author, Creator and Maintainer
- **Ronaldson Bellande**

# API Usability
```
{
  "license": [
    "Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande",
    "This program is free software: you can redistribute it and/or modify",
    "it under the terms of the GNU General Public License as published by",
    "the Free Software Foundation, either version 3 of the License, or",
    "(at your option) any later version.",
    "",
    "This program is distributed in the hope that it will be useful,",
    "but WITHOUT ANY WARRANTY; without even the implied warranty of",
    "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
    "GNU General Public License for more details.",
    "",
    "You should have received a copy of the GNU General Public License",
    "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
    "GNU General Public License v3.0 or later"
  ],
  "url": "https://bellande-robotics-sensors-research-innovation-center.org",
  "endpoint_path": {
    "bellande_probabilistic": "/api/Bellande_Probability/bellande_probability"
  },
  "Bellande_Framework_Access_Key": "bellande_web_api_opensource"
}
```
# API Payload Example
```
{
    "mu_func": "i * 0.5",
    "sigma_func": "1 + i * 0.1",
    "x": [1.0, 2.0, 3.0],
    "dimensions": 3,
    "auth": {
      "authorization_key": "bellande_web_api_opensource"
    }
}
```

# 🧙 Website Bellande API Testing 
- [![Website API Testing](https://img.shields.io/badge/Bellande%20API-Testing-0099cc?style=for-the-badge)](https://bellanderoboticssensorsresearchinnovationcenterwebsite-kot42qxp.b4a.run/api/bellande_probabilistic_experiment)
  
# Quick Bellande API Testing
```
curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Probability/bellande_probability' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "mu_func": "i * 0.5",
    "sigma_func": "1 + i * 0.1",
    "x": [1.0, 2.0, 3.0],
    "dimensions": 3,
    "auth": {
      "authorization_key": "bellande_web_api_opensource"
    }
  }'
```

# Bellande Limit Usage

## Website Crates
- https://crates.io/crates/bellande_probability

### Installation
- `cargo add bellande_probability`

## Website PYPI
- https://pypi.org/project/bellande_probability

### Installation
- `$ pip install bellande_probability`

### Usage 
```
bellande_distribution \
    --mu-func "i * 2" \
    --sigma-func "1 + i * 0.1" \
    --x "[1.0, 2.0, 3.0]" \
    --dimensions 3
```

### Upgrade (if not upgraded)
- `$ pip install --upgrade bellande_probability`

```
Name: bellande_probability
Summary: Computes the next step towards a target node
Home-page: github.com/RonaldsonBellande/bellande_probability
Author: Ronaldson Bellande
Author-email: ronaldsonbellande@gmail.com
License: GNU General Public License v3.0
```

## Published Paper
```
Coming Soon
```

## License
This Algorithm or Models is distributed under the [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/), see [LICENSE](https://github.com/RonaldsonBellande/bellande_probabilistic/blob/main/LICENSE) and [NOTICE](https://github.com/RonaldsonBellande/bellande_probabilistic/blob/main/LICENSE) for more information.

