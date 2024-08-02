
import toml
from atelier.models.autoencoder import Autoencoder
from atelier.data.datasets import generate_ts_1
hiper_params = toml.load("examples/forecasting/e1_volatility.toml")
print(hiper_params)

# -- Example data
x_data = generate_ts_1(100, [0.0, 1.0])
y_data = generate_ts_1(100, [1.0, 2.0])

