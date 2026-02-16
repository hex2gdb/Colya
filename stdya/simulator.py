import subprocess
import time


# Updated simulator.py with colors
CYAN = "\033[96m"
MAGENTA = "\033[95m"
END = "\033[0m"

def start_node(node_id, port):
    print(f"{CYAN}[*] Launching Node {node_id} on port {port}...{END}")
    return subprocess.Popen(["./target/debug/stdya", "--port", str(port), "--id", str(node_id)])

# ... rest of your code ...
print(f"{MAGENTA}[+] Network Active. Testing S-BFT Handshake...{END}")



def start_node(node_id, port):
    # This calls your native Rust binary
    print(f"[*] Launching Node {node_id} on port {port}...")
    return subprocess.Popen(["./target/release/stdya", "--port", str(port), "--id", str(node_id)])

# Simulation: 4 Nodes (3 Honest, 1 Malicious/Offline)
nodes = []
for i in range(1, 5):
    nodes.append(start_node(i, 5000 + i))

time.sleep(2)
print("[+] Network Active. Testing S-BFT Handshake...")

# Cleanup
for n in nodes:
    n.terminate()

