from setuptools import setup, find_packages

setup(name="rusty_demo",
      verstion="0.1",
      packages=find_packages(include=["rust_demo", "rust_demo.*"]))
