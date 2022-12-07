openssl genrsa -out etc/jwt-prikey.pem
openssl rsa -in etc/jwt-prikey.pem -pubout -out etc/jwt-pubkey.pem