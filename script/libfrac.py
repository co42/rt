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

def compute(iw, ih, fsx, fsy, fpx, fpy, maxiter, aa, only_color):
    # Compute image
    print('Computing image ...', file=sys.stderr)
    img = []
    for py in range(ih):
        line = []
        for px in range(iw):
            colors = [0, 0, 0]
            smooths = 0
            for _ in range(aa):
                rfsx = fsx[py % len(fsx)]
                x, y = rfsx + px * fpx, fsy + py * fpy
                x, y = uniform(x, x + fpx), uniform(y, y + fpy)
                iter, z = mandelbrot(x, y, maxiter)
                if iter < maxiter:
                    smooth = iter + 1 - log(log(abs(z))) / log(2)
                    smooth /= maxiter
                    color = hsv_to_rgb(0.95 + 10 * smooth, 0.6, 1)
                    colors[0] += color[0]
                    colors[1] += color[1]
                    colors[2] += color[2]
                    smooths += smooth
            colors[0] = colors[0] / aa
            colors[1] = colors[1] / aa
            colors[2] = colors[2] / aa
            smooths /= aa

            if only_color:
                colors[0] *= 255
                colors[1] *= 255
                colors[2] *= 255
                line += colors
            else:
                line.append((colors, smooths))

        img.append(line)
        print('\r%d%%' % int(py * 100 / ih), end='', file=sys.stderr)

    print('\rDone !', file=sys.stderr)
    return img
