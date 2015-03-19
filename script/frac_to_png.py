#!/usr/bin/python3

import sys
import readline
import traceback
import png
from librt import *
from libfrac import *

iw, ih = 0, 0
aa = 0
maxiter = 0

# Parameters: image width and height
def frac_to_png():
    # Fractal center
    fx, fy = -0.0140625, 0.7154296875
    # Fractal width
    fw = 0.005859375

    while True:
        # Fractal height
        fh = fw * ih / iw
        # Fractal start and step
        fsx, fsy = fx - fw / 2, fy - fh / 2
        fpx, fpy = fw / iw, fh / ih

        # Compute and save image
        img = compute(iw, ih, (fsx, ), (fsy, ), fpx, fpy, maxiter, aa, True)
        png.from_array(img, 'RGB').save("test.png")

        # Read input
        with ReadChar() as rc:
            cmd = rc

        # Quit
        if cmd == '\x1b':
            break
        # Eval
        elif cmd == '`':
            stmt = 'global iw, ih, maxiter, aa; ' + input('eval> ')
            try:
                exec(stmt, globals(), locals())
            except Exception as e:
                print('\033[93m' + traceback.format_exc() + '\033[0m')
        # Move
        elif cmd == 'w':
            fy -= fw / 10
        elif cmd == 's':
            fy += fw / 10
        elif cmd == 'a':
            fx -= fw / 10
        elif cmd == 'd':
            fx += fw / 10
        # Zoom
        elif cmd == 'q':
            fw /= 2
        elif cmd == 'e':
            fw *= 2
        # Print
        elif cmd == 'p':
            print('center = {}, {} size = {}, {}'.format(fx, fy, fw, fh))

if __name__ == '__main__':
    if len(sys.argv) != 5:
        print('Usage: ./frac_to_png.py width height aa maxiter')
        sys.exit(1)
    iw = int(sys.argv[1])
    ih = int(sys.argv[2])
    aa = int(sys.argv[3])
    maxiter = int(sys.argv[4])
    frac_to_png()
