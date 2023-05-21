import sys
import hashlib

if len(sys.argv) != 2:
    print("Usage: python 02_crack_numeric_password.py hash")
    exit(1)
hash = sys.argv[1]

for password in range(100000000): # 1e8 possibilities, takes about 5 minutes to run
    if hashlib.sha256(str(password).encode('UTF-8')).hexdigest() == hash:
        print("The password is:")
        print(password)
        exit(0)

print("Password not found")
