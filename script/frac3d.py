#!/usr/bin/python3

import sys
import json
from math import tan, pi
from random import uniform

def get_config():
    return {
        "picture": {
            "w": 1920,
            "h": 1050,
            "path": "image/frac3d.png"
        },
        "eye": {
            "fov": 2.1,
            "dir": {
                "y": 0.0,
                "x": -1.0999999999999996,
                "z": 0
            },
            "pos": {
                "y": 155,
                "x": -5,
                "z": 100
            }
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
                            "y": 80,
                            "z": 60
                        }
                    }
                }
            ]
        }
    }

def mandelbrot(x, y, maxiter):
    z = complex(0, 0)
    c = complex(x, y)
    iter = 0
    while iter < maxiter and z.real * z.real + z.imag * z.imag < 4.:
        z = z * z + c
        iter += 1
    return (iter, z)

def get_var(x, y):
    MAX_ITER=100
    (iter, z) = mandelbrot(x, y, MAX_ITER)
    return (iter / MAX_ITER, { "r": iter / MAX_ITER, "g": iter / MAX_ITER, "b": iter / MAX_ITER })

def frac3d():
    config = get_config()

    COUNT_X = 90
    COUNT_Z = 90
    SZ = 3
    MAX_H = 50

    FRAC_RATIO = 3 / (COUNT_X * SZ)

    for z in range(-COUNT_Z // 2, COUNT_Z // 2):
        for x in range(-COUNT_X // 2, COUNT_X // 2):
            xpos = x * SZ + (z % 2) * SZ / 2.
            zpos = z * tan(pi / 3) * SZ / 2.
            (height, color) = get_var(xpos * FRAC_RATIO, zpos * FRAC_RATIO)
            hexa = {
                "aahexa": {
                    "pos": { "x": xpos, "y": 0, "z": zpos },
                    "x": SZ,
                    "y": height * MAX_H,
                    "mat": {
                        "color": color,
                        "spec": 0.4,
                        "diff": 1
                    }
                }
            }
            config['scene']['objects'].append(hexa)

    print(json.dumps(config, indent=4))

if __name__ == '__main__':
    if len(sys.argv) != 1:
        print('Usage: ./frac3d.py')
        sys.exit(1)
    frac3d()
