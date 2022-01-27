#!/usr/bin/env bash

set -o allexport; source .env; set +o allexport

introspect() {
  rover subgraph introspect ${1} | rover subgraph publish Ocrateris-Serverless-GraphQL@node-gateway \
    --name ${2} --routing-url ${1} --schema -

}

rm -rf schemas
mkdir schemas

for row in $(cat ./config/subgraph-list.json | jq -r '.[] | @base64'); do
  _jq() {
    echo ${row} | base64 --decode | jq -r ${1}
  }
  name=$(echo $(_jq '.name'))
  endpoint=$(echo $(_jq '.endpoint'))
  introspect $endpoint $name
done




