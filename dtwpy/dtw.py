#
# Copyright (C) 2024 dtw.py.
#
# dtw.py is free software; you can redistribute it and/or modify it
# under the terms of the MIT License; see LICENSE file for more details.
#

"""DTW distance module."""

from dtwpy._internal import _dtw


def dtw(ts_a, ts_b):
    """Calculate Dynamic Time Warping (DTW) distance.
    
    Args:
        ts_a (list): Time-series A data
        ts_b (list): Time-series B data

    Returns:
        float: DTW Distance between `ts_a` and `ts_b`.
    """
    return _dtw(ts_a, ts_b)
