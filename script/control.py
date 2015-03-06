#!/usr/bin/python3

import sys
import json
from librt import *

def control(path_conf):
    with open(path_conf, 'r') as f:
        conf = json.loads(f.read())
        # o = conf['scene']['lights'][0]['bulb']
        o = conf['eye']

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
            o['pos']['z'] -= DEP
        elif cmd == 's':
            o['pos']['z'] += DEP
        elif cmd == 'a':
            o['pos']['x'] -= DEP
        elif cmd == 'd':
            o['pos']['x'] += DEP
        elif cmd == 'q':
            o['pos']['y'] += DEP
        elif cmd == 'e':
            o['pos']['y'] -= DEP
        # Rotate
        elif cmd == 'o':
            o['dir']['x'] -= ROT
        elif cmd == 'l':
            o['dir']['x'] += ROT
        elif cmd == 'k':
            o['dir']['y'] += ROT
        elif cmd == ';':
            o['dir']['y'] -= ROT
        elif cmd == 'i':
            o['dir']['z'] += ROT
        elif cmd == 'p':
            o['dir']['z'] -= ROT
        # Save
        elif cmd == 'n':
            path = input('path> ')
            with open(path, 'w+') as f:
                f.write(json.dumps(conf, indent=4))
        # Ignored
        else:
            print('Command ignored')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ./control.py [scene.json]')
        sys.exit(1)
    control(sys.argv[1])
