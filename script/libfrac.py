import sys
from math import log
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

def is_set(img, d, x, y):
    if y > 0 and y < len(img):
        line = img[y]
        if x > 0 and x < len(line):
            return line[x] is not None and line[x] == d
    return False

def distance(img_in, iw, ih, maxiter):
    print('Computing distance ...', file=sys.stderr)

    img = [[None] * iw for _ in range(ih)]
    done = 0

    for y in range(ih):
        for x in range(iw):
            if img_in[y][x][0] == maxiter:
                img[y][x] = 0
                done += 1

    d = 1
    while done < ih * iw:
        for y in range(ih):
            for x in range(iw):
                if img[y][x] is not None:
                    continue
                if any((is_set(img, d - 1, x - 1, y - 1), is_set(img, d - 1, x, y - 1), is_set(img, d - 1, x + 1, y - 1), is_set(img, d - 1, x - 1, y), is_set(img, d - 1, x + 1, y), is_set(img, d - 1, x - 1, y + 1), is_set(img, d - 1, x, y + 1), is_set(img, d - 1, x + 1, y + 1))):
                    img[y][x] = d
                    done += 1
        d += 1
        print('\r%d' % d, end='', file=sys.stderr)

    print('\nDone !    ', file=sys.stderr)
    return (img, d)
