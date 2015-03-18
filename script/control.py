#!/usr/bin/python3

import readline
import sys
import traceback
import json
from librt import *

actions = {}

def badd(root, key, value):
    root[key] += value

def bmove(root, move):
    actions['w'] = lambda: badd(root['pos'], 'z', -move)
    actions['s'] = lambda: badd(root['pos'], 'z', +move)
    actions['a'] = lambda: badd(root['pos'], 'x', -move)
    actions['d'] = lambda: badd(root['pos'], 'x', +move)
    actions['q'] = lambda: badd(root['pos'], 'y', +move)
    actions['e'] = lambda: badd(root['pos'], 'y', -move)

def brot(root, rot):
    actions['o'] = lambda: badd(root['dir'], 'x', -rot)
    actions['l'] = lambda: badd(root['dir'], 'x', +rot)
    actions['k'] = lambda: badd(root['dir'], 'y', +rot)
    actions[';'] = lambda: badd(root['dir'], 'y', -rot)
    actions['i'] = lambda: badd(root['dir'], 'z', +rot)
    actions['p'] = lambda: badd(root['dir'], 'z', -rot)

def bmr(root, move, rot):
    bmove(root, move)
    brot(root, rot)

def sscreen(conf, w, h):
    conf['picture']['w'] = w
    conf['picture']['h'] = h

def init_actions(conf):
    bmr(conf['eye'], 5, 0.1)

def control(path_conf):
    with open(path_conf, 'r') as f:
        conf = json.loads(f.read())
        init_actions(conf)

    while True:
        print('Updating image ...')
        call_rt(conf)
        print('Image updated !')

        # Read input
        with ReadChar() as rc:
            cmd = rc

        # Quit
        if cmd == '\x1b':
            break
        # Eval
        elif cmd == '`':
            stmt = input('eval> ')
            try:
                eval(stmt)
            except Exception as e:
                print('\033[93m' + traceback.format_exc() + '\033[0m')
        # Save
        elif cmd == 'n':
            path = input('path> ')
            with open(path, 'w+') as f:
                f.write(json.dumps(conf, indent=4))
        # Actions
        elif cmd in actions:
            try:
                actions[cmd]()
            except Exception as e:
                print('\033[93m' + traceback.format_exc() + '\033[0m')
        # Action ignored
        else:
            print('Command ignored')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ./control.py [scene.json]')
        sys.exit(1)
    control(sys.argv[1])
