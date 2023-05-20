import requests
import json

def getStock(URL):
    page = requests.get(URL)
    data = json.loads(page.content)
    if 'available' in data:
        return True
    return False

def getPI400():
    return getStock("https://shop.pimoroni.com/products/raspberry-pi-400-personal-computer-kit.js")

if __name__ == "__main__":
    if getPI400():
        print('PI 400 is in Stock at Pimoroni')
