import requests
import json

def getStock(URL):
    page = requests.get(URL)
    if page.status_code == 200:
        try:
            data = json.loads(page.content)
            if 'available' in data:
                return True
        except json.JSONDecodeError as error:
            print('Error decoding json: ' + error.msg)
            print(page.content)
    return False

def getPI400():
    return getStock("https://shop.pimoroni.com/products/raspberry-pi-400-personal-computer-kit.js")

def getPIZERO():
    return getStock("https://shop.pimoroni.com/product/raspberry-pi-zero-essentials-kit.js")

if __name__ == "__main__":
    if getPI400():
        print('PI 400 is in Stock at Pimoroni')
    else:
        print('PI 400 is not in Stock at Pimoroni')
    if getPIZERO():
        print('PI Zero Kit is in Stock at Pimoroni')
    else:
        print('PI Zero Kit is not in Stock at Pimoroni')