#!/usr/bin/python3

import sys
import json
from math import tan, pi
from random import uniform

def get_config():
    return {
        "picture": {
            "w": 200,
            "h": 200,
            "path": "image/hexaground.png"
        },
        "eye": {
            "pos": { "x": -5, "y": 20, "z": 35 },
            "dir": { "x": -0.7, "y": -0.1, "z": 0 },
            "fov": 2.1
        },
        "scene": {
            "objects": [
            ],
            "lights": [
                {
                    "bulb": {
                        "diff": 0.9,
                        "spec": 1.5,
                        "shin": 20,
                        "pos": {
                            "x": 25,
                            "y": 50,
                            "z": 5
                        }
                    }
                }
            ]
        }
    }

def get_color():
    return { "r": uniform(0, 1), "g": uniform(0, 1), "b": uniform(0, 1) }

def get_height(min, max):
    return uniform(min, max)

def hexaground():
    config = get_config()

    COUNT_X=8
    COUNT_Z=10
    SZ=10
    MIN_H=1
    MAX_H=8

    for z in range(-COUNT_Z // 2, COUNT_Z // 2):
        for x in range(-COUNT_X // 2, COUNT_X // 2):
            hexa = {
                "aahexa": {
                    "pos": { "x": x * SZ + (z % 2) * SZ / 2., "y": 0, "z": z * tan(pi / 3) * SZ / 2. },
                    "x": SZ,
                    "y": get_height(MIN_H, MAX_H),
                    "mat": {
                        "color": get_color(),
                        "spec": 0.4,
                        "diff": 1,
                        "refl": 0.1
                    }
                }
            }
            config['scene']['objects'].append(hexa)

    print(json.dumps(config, indent=4))

if __name__ == '__main__':
    if len(sys.argv) != 1:
        print('Usage: ./hexaground.py')
        sys.exit(1)
    hexaground()
