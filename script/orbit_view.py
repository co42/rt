#!/usr/bin/python3

import sys
import json
from math import *
from librt import *

def get_node(path, node):
    if type(path) is str:
        path = path.split('.')
    if len(path) == 0:
        return node

    cur = path[0]
    if cur.isdecimal():
        cur = int(cur)
    node = node[cur]
    return get_node(path[1:], node)

def orbit_view(scene, pos_path, distance, count):
    with open(scene, 'r') as f:
        conf = json.loads(f.read())
        look_at = get_node(pos_path, conf)

    step = -(2 * pi) / count
    for cur in range(count):
        angle = step * cur
        conf['eye']['dir'] = { 'x': angle, 'y': 0, 'z': 0 }
        x = look_at['x']
        y = look_at['y'] - sin(angle) * distance
        z = look_at['z'] + cos(angle) * distance
        conf['eye']['pos'] = { 'x': x, 'y': y, 'z': z }
        conf['picture']['path'] = "image/orbit_view_{:03}.png".format(cur + 1)

        print('\r{}/{}'.format(cur + 1, count), end='')
        call_rt(conf)
    print()

if __name__ == '__main__':
    if len(sys.argv) != 5:
        print('Usage: ./orbit_view.py scene.json pos.path distance count')
        sys.exit(1)
    scene = sys.argv[1]
    pos_path = sys.argv[2]
    distance = float(sys.argv[3])
    count = int(sys.argv[4])
    orbit_view(scene, pos_path, distance, count)
