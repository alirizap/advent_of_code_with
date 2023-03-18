n = int(input())
name_and_address_list = []
for _ in range(n):
    name_and_address_list.append(input())

t = int(input())
name_and_params_list = []
for _ in range(t):
    name_and_params_list.append(input())


urls = {}

for line in name_and_address_list:
    name, url = line.split(" ")
    url = url.replace("<", "{")
    url = url.replace(">", "}")
    urls[name] = url 


for line in name_and_params_list:
    name = line.split(" ")[0]
    url = urls.get(name, None)
    if url == None:
        print("[Error] url not found")
    else:
        params = {}
        for param in line.split(" ")[1:]:
            name, value = param.split("=")
            params[name] = value 

        try:
            print(url.format(**params))
        except KeyError:
            print("[Error] missing parameter(s)")

    