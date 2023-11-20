
# Schemars
[![PyPI version](https://badge.fury.io/py/schemars.svg)](https://badge.fury.io/py/schemars)
[![Downloads](https://static.pepy.tech/personalized-badge/schemars?period=month&units=international_system&left_color=black&right_color=brightgreen&left_text=downloads/month)](https://pepy.tech/project/schemars)

## Introduction
Schemars is a Python package, written in Rust and leveraging PyO3, designed for efficient and flexible serialization of Python class instances. It provides a simple yet powerful schema-based approach to serialize complex Python objects.

## Installation
To install Schemars, run the following command:
```
pip install schemars
```

## Requirements
- Python 3.x
- Rust (optional for development)

## Usage
To use Schemars, define your Python class and corresponding schema class, then serialize instances as shown below:

```python
class Product:
    def __init__(self, name, price, created):
        self.name = name
        self.price = price
        self.created = created

product = Product("test", 10, "1577836800")

import schemars
class ProductSchema(schemars.Schema):
    name = schemars.Str(strict=True)
    price = schemars.Decimal()
    created = schemars.Date(format='%Y/%m/%d')

print(ProductSchema().serialize(product))
```

## Documentation Coming Soon!
We are currently working on comprehensive documentation for Schemars, which will cover detailed usage, advanced features, and integration guides. Stay tuned for updates, and feel free to reach out to us with any specific questions or suggestions in the meantime.

## Upcoming in Version 1.0
In the upcoming Version 1.0 of Schemars, we will be introducing additional functionalities including both validation and deserialization. This enhancement aims to provide a more comprehensive and robust experience in handling and processing Python class instances.

## Inspired by Marshmallow and Django REST Framework
Schemars was developed in response to performance challenges encountered with existing serialization tools like Marshmallow and Django REST Framework. Our goal was to create a solution that not only addresses these performance issues but also remains user-friendly and familiar.

## Easy Migration
If you're already using Marshmallow or Django REST Framework, you'll find Schemars's syntax and usage comfortably similar. This design choice is intentional to ensure that migration to Schemars is smooth and can be accomplished in just minutes. We have prioritized maintaining a familiar interface while significantly enhancing performance, so you can switch to Schemars with minimal adjustments to your existing codebase.

# Benchmarking Schemars

## Quick Benchmark Setup
To compare Schemars with Django REST Framework (DRF), Marshmallow and Pydantic, follow these steps:

### Installation
Install benchmarking requirements:
```bash
python3 -m pip install -r bench/requirements.txt
```

### Running the Benchmark
Execute the benchmark script:
```bash
python3 bench/run.py
```

This will run serialization tasks using Schemars, Marshmallow, DRF and Pydantic, allowing you to directly compare their performance.
## License
Schemars is released under the [MIT License]
