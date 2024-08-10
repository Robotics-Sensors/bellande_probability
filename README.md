# 📦 Bellande Probabilistic(Infinite Dimentions)

## 🧙 Organization Website
- [![Organization Website](https://img.shields.io/badge/Explore%20Our-Website-0099cc?style=for-the-badge)](https://robotics-sensors.github.io)


## 🧙 Organization Github
- [![Organization Github ](https://img.shields.io/badge/Explore%20Our-Github-0099cc?style=for-the-badge)](https://github.com/Robotics-Sensors)

# Author, Creator and Maintainer
- **Ronaldson Bellande**

# API Usability
```
{
  "url": "https://bellanderoboticssensorsresearchinnovationcenter-kot42qxp.b4a.run",
  "endpoint_path": {
    "bellande_probabilistic": "/api/Bellande_Probabilistic/bellande_probabilistic_nd"
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
  'http://localhost:4000/api/Bellande_Distribution/bellande_distribution_function_nd' \
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


## Published Paper
```
Coming Soon
```

## License
This Algorithm or Models is distributed under the [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/), see [LICENSE](https://github.com/RonaldsonBellande/bellande_probabilistic/blob/main/LICENSE) and [NOTICE](https://github.com/RonaldsonBellande/bellande_probabilistic/blob/main/LICENSE) for more information.

