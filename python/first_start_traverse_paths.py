import hvac


def execute(mount_point):

    def traverse(path):
        # The mount point used here is the one from our very first tutorial.
        secrets = client.kv.v1.list_secrets(mount_point=mp, path=path)
        paths = secrets["data"]["keys"]

        for nextp in paths:
            if nextp[-1] == "/":  # Assumption: only non-terminal end in this.
                traverse(path + nextp)
            else:
                print(" ", path + nextp)

    print(f"Paths under {mount_point}")
    mp = mount_point
    traverse("")


client = hvac.Client()
client.kv.default_kv_version = 1

execute(mount_point="kv-v1")
