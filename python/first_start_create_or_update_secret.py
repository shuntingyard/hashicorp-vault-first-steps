import uuid

import hvac


def execute():
    client.kv.v1.create_or_update_secret(
        mount_point="kv-v1",
        path="eng/apikey/Twitter",
        # Don't invent stuff manually.
        secret={"key": str(uuid.uuid4())},
    )


client = hvac.Client()
client.kv.default_kv_version = 1
execute()
