# vpnapi-cli
A tool to query for IPs on vpnapi.io


## Why?
You don't need to use this, you can straight up just use curl
`curl "https://vpnapi.io/api/{ip_address}?key={api_key}"`

I'm writing this project to learn Rust


## How to use?
First, you will need a API key, create a account on vpnapi.io
The free account gives you 1000 credits/day

You can save your key
`vpnapi-cli config` -> Enter your key and press enter

Now you can just query for IPs
`vpnapi-cli 1.1.1.1`

Or, if you don't want to save your key:
`vpnapi-cli 8.8.8.8 --key {api_key}`

If you want the result to be more readable, enter `-p | --pprint`
`vpnapi-cli 8.8.8.8 --pprint`