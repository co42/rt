import sys
from math import log, sqrt
from colorsys import hsv_to_rgb
from random import uniform

def mandelbrot(x, y, maxiter):
    z = complex(0, 0)
    c = complex(x, y)
    iter = 0
    while iter < maxiter and z.real * z.real + z.imag * z.imag < 4.:
        z = z * z + c
        iter += 1
    return iter, z

def compute(iw, ih, fsx, fsy, fpx, fpy, maxiter, aa):
    # Compute image
    print('Computing fractal ...', file=sys.stderr)
    img = []
    for py in range(ih):
        line = []
        for px in range(iw):
            iter = 0
            z = 0
            for _ in range(aa):
                x, y = fsx + px * fpx, fsy + py * fpy
                x, y = uniform(x, x + fpx), uniform(y, y + fpy)
                iter_aa, z_aa = mandelbrot(x, y, maxiter)
                iter += iter_aa
                z += z_aa
            iter /= aa
            z /= aa
            line.append((iter, z))

        img.append(line)
        print('\r%d%%' % int(py * 100 / ih), end='', file=sys.stderr)

    print('\rDone !', file=sys.stderr)
    return img

def nearest(base, iw, ih, x, y):
    dist = None
    for ny in range(ih):
        for nx in range(iw):
            if base[ny][nx]:
                dx = nx - x
                dy = ny - y
                new_dist = sqrt(dx * dx + dy * dy)
                if dist is None or new_dist < dist:
                    dist = new_dist
    return dist

def distance(img_in, iw, ih, maxiter):
    print('Computing distance ...', file=sys.stderr)

    base = []
    for y in range(ih):
        line = []
        for x in range(iw):
            if img_in[y][x][0] == maxiter:
                line.append(True)
            else:
                line.append(False)
        base.append(line)

    img = []
    dist_max = 0
    for y in range(ih):
        line = []
        for x in range(iw):
            dist = nearest(base, iw, ih, x, y)
            dist_max = max(dist, dist_max)
            line.append(dist)
        img.append(line)
        print('\r%d%%' % int(y * 100 / ih), end='', file=sys.stderr)

    print('\nDone !    ', file=sys.stderr)
    return (img, dist_max)
