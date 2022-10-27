import json

import hvac
from hvac.exceptions import InvalidPath


def execute():
    for path in [
        "prod/cert/mysql",
        "eng/apikey/Google",
    ]:
        try:
            response = client.kv.v1.read_secret(mount_point="kv-v1", path=path)
            print(json.dumps(response))
        except InvalidPath as e:
            print(f"Invalid Path {e}\n")


client = hvac.Client()
client.kv.default_kv_version = 1
execute()
