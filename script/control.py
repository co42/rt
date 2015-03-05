#!/usr/bin/python3

import sys
import json
from librt import *

def control(path_conf):
    with open(path_conf, 'r') as f:
        conf = json.loads(f.read())

    DEP = 5
    ROT = 0.1

    while True:
        print('Updating image ...')
        call_rt(conf)
        print('Image updated !')

        # Read input
        with ReadChar() as rc:
            cmd = rc

        # Quit
        if cmd == '`':
            break
        # Move
        elif cmd == 'w':
            conf['eye']['pos']['z'] -= DEP
        elif cmd == 's':
            conf['eye']['pos']['z'] += DEP
        elif cmd == 'a':
            conf['eye']['pos']['x'] -= DEP
        elif cmd == 'd':
            conf['eye']['pos']['x'] += DEP
        elif cmd == 'q':
            conf['eye']['pos']['y'] += DEP
        elif cmd == 'e':
            conf['eye']['pos']['y'] -= DEP
        # Rotate
        elif cmd == 'o':
            conf['eye']['dir']['x'] -= ROT
        elif cmd == 'l':
            conf['eye']['dir']['x'] += ROT
        elif cmd == 'k':
            conf['eye']['dir']['y'] += ROT
        elif cmd == ';':
            conf['eye']['dir']['y'] -= ROT
        elif cmd == 'i':
            conf['eye']['dir']['z'] += ROT
        elif cmd == 'p':
            conf['eye']['dir']['z'] -= ROT
        # Ignored
        else:
            print('Command ignored')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ./control.py [config.json]')
        sys.exit(1)
    control(sys.argv[1])
