import subprocess
import os
import fcntl
import time

# Force unbuffered environment
os.environ["PYTHONUNBUFFERED"] = "1"

def set_nonblocking(fd):
    flags = fcntl.fcntl(fd, fcntl.F_GETFL)
    fcntl.fcntl(fd, fcntl.F_SETFL, flags | os.O_NONBLOCK)

procs = []
for i in range(1, 5):
    p = subprocess.Popen(
        ["./target/release/stdya", "--port", str(5000+i), "--id", str(i)],
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True
    )
    set_nonblocking(p.stdout)
    procs.append(p)

print("\033[95m[+] S-BFT Quorum Online. Streaming live logs...\033[0m")

start_time = time.time()
while time.time() - start_time < 30:
    for i, p in enumerate(procs):
        try:
            line = p.stdout.readline()
            if line:
                print(f"Node {i+1}: {line.strip()}")
        except EOFError:
            pass
        except Exception:
            pass
    time.sleep(0.1) # Small sleep to prevent 100% CPU usage

for p in procs:
    p.terminate()

