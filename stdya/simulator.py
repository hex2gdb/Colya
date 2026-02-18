import socket
import struct
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ed25519

def send_signed_vote(target_port, node_id, block_hash="BLOCK_001"):
    # 1. Generate Ed25519 keys
    private_key = ed25519.Ed25519PrivateKey.generate()
    public_key_obj = private_key.public_key()
    
    # FIX: Use standard serialization for raw bytes
    public_key_bytes = public_key_obj.public_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PublicFormat.Raw
    )
    
    # 2. Sign the message (returns 64 bytes)
    signature = private_key.sign(block_hash.encode())

    # 3. Pack the payload: 4-byte Int (ID) + 32-byte PubKey + 64-byte Signature
    # '!I' ensures Big Endian (Network) byte order for the ID
    payload = struct.pack('!I', node_id) + public_key_bytes + signature
    
    # 4. Send to the Rust Node
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect(("127.0.0.1", target_port))
            s.sendall(payload)
            print(f"✅ Sent 100-byte signed payload for Node {node_id} to port {target_port}")
    except ConnectionRefusedError:
        print(f"❌ Connection refused on port {target_port}. Is the Rust node running?")

if __name__ == "__main__":
    send_signed_vote(5001, 2)

