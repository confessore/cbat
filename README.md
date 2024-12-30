Welcome to the unofficial Coinbase Advanced Trade API for rust

Set your environment variables
The secret must contain EC as it is a sec1 key and needs to be converted to pkcs#8
```
CBAT_KEY_NAME="organizations/{org_id}/apiKeys/{key_id}"
CBAT_KEY_SECRET="-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
```