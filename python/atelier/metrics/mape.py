
# --- ----------------------------------------------------------------------- #
# --- File: mape.py
# --- ----------------------------------------------------------------------- #

import torch
from torch import Tensor
from typing import Tuple, Union
from atelier.utilities import checks

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def _mape_update(
    preds: Tensor,
    target: Tensor,
    epsilon: float = 1e-06,
) -> Tuple[Tensor, int]:

    """Update and returns variables required to compute Mean Percentage Error.

    Check for same shape of input tensors.

    Args:
        preds: Predicted tensor
        target: Ground truth tensor
        epsilon: Specifies the lower bound for target values. 
                 Any target value below epsilon is set to epsilon 
                 (avoids ``ZeroDivisionError``).

    """

    checks.check_same_shape(preds, target)

    abs_diff = torch.abs(preds - target)
    abs_per_error = abs_diff / torch.clamp(torch.abs(target), min=epsilon)
    sum_abs_per_error = torch.sum(abs_per_error)
    num_obs = target.numel()

    return sum_abs_per_error, num_obs

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def _mape_compute(
        sum_abs_per_error: Tensor,
        num_obs: Union[int, Tensor]
    ) -> Tensor:
    
    """Compute Mean Absolute Percentage Error.

    Args:
        sum_abs_per_error: Sum of absolute value of percentage errors over 
                           all observations
        num_obs: Number of predictions or observations

    Example:
        >>> target = torch.tensor([1, 10, 1e6])
        >>> preds = torch.tensor([0.9, 15, 1.2e6])
        >>> sum_abs_per_error, num_obs = mape_update(preds, target)
        >>> _mean_absolute_percentage_error_compute(sum_abs_per_error, num_obs)
        tensor(0.2667)

    """

    return sum_abs_per_error / num_obs

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def mape(preds: Tensor, target: Tensor) -> Tensor:
    """Compute mean absolute percentage error.

    Args:
        preds: estimated labels
        target: ground truth labels

    Return:
        Tensor with MAPE

    Note:
        The epsilon value is taken from `scikit-learn's implementation of MAPE`_.

    Example:
        >>> from torchmetrics.functional.regression import mean_absolute_percentage_error
        >>> target = torch.tensor([1, 10, 1e6])
        >>> preds = torch.tensor([0.9, 15, 1.2e6])
        >>> mean_absolute_percentage_error(preds, target)
        tensor(0.2667)

    """
    
    sum_abs_per_error, num_obs = _mape_update(preds, target)
    
    return _mape_compute(sum_abs_per_error, num_obs)


