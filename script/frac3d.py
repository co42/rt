#!/usr/bin/python3

import sys
import json
from math import tan, pi
from libfrac import *

def get_config():
    return {
        "picture": {
            "w": 100,
            "h": 100,
            "path": "image/frac3d.png"
        },
        "eye": {
            "fov": 2.1,
            "dir": {
                "y": 0.0,
                "x": -pi / 6,
                "z": 0
            },
            "pos": {
                "y": 50,
                "x": 0,
                "z": 170
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
                            "x": 0,
                            "y": 300,
                            "z": 200
                        }
                    }
                }
            ]
        }
    }

def frac3d():
    config = get_config()

    # Image
    iw = 100
    ih = 100
    aa = 20
    maxiter = 50

    # Hexa
    hlarge = 5
    hheight = 250
    hw = iw * hlarge
    hh = ih * hlarge

    # Hexa start and step
    hsx, hsy = -hw / 2, -hh / 2
    hpx, hpy = hlarge, tan(pi / 3) * hlarge / 2

    # Fractal center
    fx, fy = -0.0140625, 0.7154296875
    # Fractal size
    fw = 0.005859375
    fh = fw * ih / iw
    # Fractal start and step
    fsx, fsy = fx - fw / 2, fy - fh / 2
    fpx = fw / iw
    fpy = tan(pi / 3) * fpx / 2

    # Compute fractal
    img = compute(iw, ih, (fsx, fsx + fpx / 2), fsy, fpx, fpy, maxiter, aa, False)
    for py in range(ih):
        for px in range(iw):
            (color, height) = img[py][px]
            x = hsx + px * hpx + (py % 2) * hlarge / 2.
            z = hsy + py * hpy
            h = hheight - height * hheight
            hexa = {
                "aahexa": {
                    "pos": { "x": x, "y": 0, "z": z },
                    "x": hlarge,
                    "y": h,
                    "mat": {
                        "color": { "r": color[0], "g": color[1], "b": color[2] },
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
