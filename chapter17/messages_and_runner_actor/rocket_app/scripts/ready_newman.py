import json


with open("./to_do_items.postman_collection.json", "r") as file:
    newman_data = json.loads(file.read())

with open("./fresh_token.json", "r") as file:
    fresh_token_data = json.loads(file.read())


newman_data["auth"]["apikey"][0]["value"] = fresh_token_data["token"]

with open("./test_newman.json", "w") as file:
    file.write(json.dumps(newman_data))
