#["auth"]["apikey"][0]["value"]

TOKEN=$(jq '.token' fresh_token.json)
jq '.auth.apikey[0].value = '"$TOKEN"'' to_do_items.postman_collection.json > test_newman.json

echo $TOKEN
jq '.auth.apikey[0]' test.json
