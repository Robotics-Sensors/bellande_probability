from setuptools import setup, find_packages

with open("../../README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="bellande_probability",
    version="0.0.3",
    description="Robots Probability",
    long_description=long_description,
    long_description_content_type="text/markdown",
    author="RonaldsonBellande",
    author_email="ronaldsonbellande@gmail.com",
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    include_package_data=True,
    install_requires=[
        "numpy",
    ],
    classifiers=[
        "License :: OSI Approved :: GNU General Public License v3 (GPLv3)",
        "Programming Language :: Python",
    ],
    keywords=["package", "setuptools"],
    python_requires=">=3.0",
    extras_require={
        "dev": ["pytest", "pytest-cov[all]", "mypy", "black"],
    },
    package_data={
        'bellande_probability': ['Bellande_Probability'],
    },
    entry_points={
        'console_scripts': [
            'bellande_probability_api = bellande_probability.bellande_probability_api:main',
        ],
    },
    project_urls={
        "Home": "https://github.com/Robotics-Sensors/bellande_probability",
        "Homepage": "https://github.com/Robotics-Sensors/bellande_probability",
        "documentation": "https://github.com/Robotics-Sensors/bellande_probability",
        "repository": "https://github.com/Robotics-Sensors/bellande_probability",
    },
)
