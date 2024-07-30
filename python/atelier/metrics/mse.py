
# --- ------------------------------------------------------------------- --- #
# --- File: mse.py
# --- ------------------------------------------------------------------- --- #

import torch
from torch import Tensor
from typing import Tuple, Union
from atelier.utilities import checks

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def _mse_update(
        preds: Tensor,
        target: Tensor,
        num_outputs: int
    ) -> Tuple[Tensor, int]:
    
    """Update and returns variables required to compute Mean Squared Error.

    Check for same shape of input tensors.

    Args:
        preds: Predicted tensor
        target: Ground truth tensor
        num_outputs: Number of outputs in multioutput setting

    """
    
    checks.check_same_shape(preds, target)
    
    if num_outputs == 1:
        preds = preds.view(-1)
        target = target.view(-1)
    diff = preds - target
    sum_squared_error = torch.sum(diff * diff, dim=0)
    
    return sum_squared_error, target.shape[0]


# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def _mse_compute(sum_squared_error: Tensor,
                 num_obs: Union[int, Tensor],
                 squared: bool = True
                 ) -> Tensor:
    
    """Compute Mean Squared Error.

    Args:
        sum_squared_error: Sum of square of errors over all observations
        num_obs: Number of predictions or observations
        squared: Returns RMSE value if set to False.

    Example:
        >>> preds = torch.tensor([0., 1, 2, 3])
        >>> target = torch.tensor([0., 1, 2, 2])
        >>> sum_squared_error, num_obs = _mean_squared_error_update(preds, target, num_outputs=1)
        >>> _mean_squared_error_compute(sum_squared_error, num_obs)
        tensor(0.2500)

    """

    return sum_squared_error / num_obs if squared else torch.sqrt(sum_squared_error / num_obs)


# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def mse(preds: Tensor, 
        target: Tensor, 
        squared: bool = True, 
        num_outputs: int = 1
        ) -> Tensor:
    
    """Compute mean squared error.

    Args:
        preds: estimated labels
        target: ground truth labels
        squared: returns RMSE value if set to False
        num_outputs: Number of outputs in multioutput setting

    Return:
        Tensor with MSE

    Example:
        >>> from torchmetrics.functional.regression import mean_squared_error
        >>> x = torch.tensor([0., 1, 2, 3])
        >>> y = torch.tensor([0., 1, 2, 2])
        >>> mean_squared_error(x, y)
        tensor(0.2500)

    """

    sum_squared_error, num_obs = _mse_update(preds,
                                             target, 
                                             num_outputs=num_outputs)

    return _mse_compute(sum_squared_error, num_obs, squared=squared)

