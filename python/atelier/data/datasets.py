
# --- ------------------------------------------------------------------- --- #
# --- datasets.py ------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

import numpy as np

def generate_ts_1(n_obs, dist_params=[0.0, 1]):
    """

    """

    tf = 80 * np.pi
    t = np.linspace(0., tf, n_obs)
    y_1 = np.sin(t) + 0.8 * np.cos(0.5*t)
    y_2 = np.random.normal(dist_params[0],
                           dist_params[1],
                           n_obs) + 2.5
    y = y_1 + y_2

    return y.tolist()

