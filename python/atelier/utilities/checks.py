
# --- ----------------------------------------------------------------------- #
# --- File: checks.py
# --- ------------------------------------------------------------------- --- #

from torch import Tensor

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def check_same_shape(preds: Tensor, target: Tensor) -> None:
    """
    Check that predictions and target have the same shape, else raise error.
    """

    txt_0 = f"Predictions and targets are expected to have the same shape" 
    txt_1 = f"got {preds.shape} and {target.shape}"

    if preds.shape != target.shape:
        raise RuntimeError(txt_0 + ", " + txt_1)

