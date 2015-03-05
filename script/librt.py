import subprocess
import json
import tty, sys, termios

def call_rt(conf):
    subprocess.check_output(['cargo run --release'], input=json.dumps(conf).encode('utf-8'), shell=True)

class ReadChar():
    def __enter__(self):
        self.fd = sys.stdin.fileno()
        self.old_settings = termios.tcgetattr(self.fd)
        tty.setraw(sys.stdin.fileno())
        return sys.stdin.read(1)

    def __exit__(self, type, value, traceback):
        termios.tcsetattr(self.fd, termios.TCSADRAIN, self.old_settings)
