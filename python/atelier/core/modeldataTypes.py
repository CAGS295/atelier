# --- ----------------------------------------------------------------------- #
# --- File: modeldataTypes.py
# --- ----------------------------------------------------------------------- #

import typing
from attrs import validators
from attrs import define, field

from atelier.core.dataTypes import tensorTypes

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

@define(slots=True, frozen=True)
class Metric:
    """
    Container class for Model's Performance Metric data
    """

    timestamp: int = field(kw_only=True, validator=validators.instance_of(int))

