from setuptools import setup, find_packages

setup(
    name="kcm",
    version="0.1.0",
    description="KCM Knowledge Columnar Model - Python SDK",
    author="KCM Team",
    license="MIT",
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    python_requires=">=3.7",
    extras_require={
        "dev": ["pytest>=7.0"],
    },
)
