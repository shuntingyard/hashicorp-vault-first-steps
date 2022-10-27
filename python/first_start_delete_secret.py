import hvac


def execute():
    client.kv.v1.delete_secret(
        mount_point="kv-v1",
        path="eng/apikey/Twitter",
    )


client = hvac.Client()
client.kv.default_kv_version = 1
execute()
