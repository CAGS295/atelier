
# atelier

A Machine Learning Infrastructure Builder for training, fine-tuning, serving and monitoring Financial Machine Learning Models for High Frequency Trading in Crypto Markets.


## Build and test locally

With the combination of `tool.setuptools-rust` and `pip`, the following line will get the Rust code compiled, and, packaged into a python package which is then installed locally
```
python -m pip install -e .
```

Then, it can be tested with something as simple as this:

```
python -c "import string_sum; print(string_sum.sum_as_string(1,2))"
```

## References

### Building for different python dist and OS
https://setuptools-rust.readthedocs.io/en/latest/building_wheels.html


