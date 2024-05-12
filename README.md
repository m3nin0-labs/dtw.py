
# dtw.py

Welcome to `dtw.py`, a high-level Python API designed to simplify the use of Dynamic Time Warping (DTW) in your projects. This library leverages a robust Rust implementation to offer fast and efficient DTW computations, making it ideal for both uni and multivariate time series analysis.

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

distance = dtw([[1, 2, 3]], [[1, 2, 3]])
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
