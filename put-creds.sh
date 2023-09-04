#!/bin/bash

const_id=$(cosmo whoami -o json | jq -r '.user.constellation_ids[0]')
password=$(jq -r '.token' ~/.cosmo/registry.token)
payload=$(jq -n --arg const_id "$const_id" --arg password "$password" '{"registry.cosmonic.sh": {"registryType": "oci", "username": $const_id, "password": $password}}')
nats=$(which nats || echo -n "~/go/bin/nats")
$nats req -s 127.0.0.1:4223 "wasmbus.ctl.$const_id.registries.put" "$payload"
