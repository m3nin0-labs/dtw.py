
# dtw.py

Welcome to `dtw.py`, a high-level Python API designed to simplify the use of Dynamic Time Warping (DTW) in your projects. This library leverages a robust Rust implementation to offer fast and efficient DTW computations, making it ideal for both uni and multivariate time series analysis.

> Note: This is a hobby project.

## What is Dynamic Time Warping?

Dynamic Time Warping (DTW) is an algorithm for measuring similarity between two time-series which may vary in speed. For instance, similarities in walking patterns could be detected, even if one person was walking faster than the other. DTW has applications in various fields such as audio and speech processing, finance, and health monitoring, where time series data is a key component.

## Installation

You can install `dtw.py` using:

```
pip install git+https://github.com/m3nin0-labs/dtw.py
```

## Usage

Using `dtw.py` is straightforward. Here's a quick example to get you started:

```python
from dtwpy import dtw

# time-series a
ts_a = [[10.2, 10.5, 10.8, 11.0, 11.2, 11.5]]

# time-series b
ts_b = [[9.8, 9.6, 9.4, 9.2, 9.0, 8.8]]

dtw(ts_a, ts_b)
#> 4.2778496742248535
```

You can use `dtw.py` to compute the DTW distance between uni or multivariate time series data.

## Features

- **Efficiency:** Built on Rust, ensuring high performance.
- **Flexibility:** Supports both univariate and multivariate time series.
- **Ease of Use:** Simple and intuitive high-level Python API.

## Contributing

We welcome contributions! If you have suggestions for improvements or bug fixes, please feel free to fork the repository and submit a pull request.

## License

`dtw.py` is distributed under the MIT license. See `LICENSE` for more details.
