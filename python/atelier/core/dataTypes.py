
# --- ----------------------------------------------------------------------- #
# --- File: dataTypes.py
# --- ----------------------------------------------------------------------- #

from typing import NewType
from attr import validators
from attrs import define
import torch

torchTensor = NewType("torchTensor", torch.Tensor())


@define(slots=True, frozen=True)
class tensorTypes:
    torchTensor = torch.Tensor()
    validator = [
        validators.instance_of(torch.Tensor),
    ]

