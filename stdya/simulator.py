import socket
import struct
import time
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ed25519

def create_tx_payload(node_id, sender, receiver, amount, nonce):
    """Packs a transaction into the 100-byte BFT protocol format."""
    # 1. Generate keys for the sender
    private_key = ed25519.Ed25519PrivateKey.generate()
    public_key_bytes = private_key.public_key().public_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PublicFormat.Raw
    )
    
    # 2. Sign a message representing the transaction
    tx_msg = f"{sender}->{receiver}:{amount}:{nonce}"
    signature = private_key.sign(tx_msg.encode())

    # 3. Pack: 4-byte ID + 32-byte PubKey + 64-byte Signature = 100 bytes
    payload = struct.pack('!I', node_id) + public_key_bytes + signature
    return payload

def stress_test_mempool(target_port, num_txs=10):
    print(f"🚀 Starting Mempool Stress Test on port {target_port}...")
    
    for i in range(num_txs):
        payload = create_tx_payload(
            node_id=2, 
            sender="User_A", 
            receiver="User_B", 
            amount=10 + i, 
            nonce=int(time.time() * 1000) + i # Unique millisecond nonce
        )
        
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.settimeout(2)
                s.connect(("127.0.0.1", target_port))
                s.sendall(payload)
                print(f"✅ Sent TX {i+1}/{num_txs} (Nonce: ...{str(i)[-4:]})")
            
            # Small delay to prevent OS socket exhaustion
            time.sleep(0.1) 
            
        except Exception as e:
            print(f"❌ Failed to send TX {i}: {e}")
            break

if __name__ == "__main__":
    # Ensure your Rust node is running on 5001 first!
    stress_test_mempool(5001, num_txs=20)

