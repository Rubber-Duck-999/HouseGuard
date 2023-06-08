#!/usr/bin/python3
'''Python script to send '''
import requests
import json
import logging
import os
import sqlite3


def get_config():
    '''Get configuration values'''
    logging.info('# get_config()')
    try:
        if not os.path.isfile('config.json'):
            return False
        config_file        = open('config.json', "r")
        config_data        = json.load(config_file)
        return config_data['topic']
    except IOError as error:
        logging.error('File not available: {}'.format(error))
    except KeyError as error:
        logging.error('Key not available: {}'.format(error))
    except TypeError as error:
        logging.error('Type not available: {}'.format(error))
    return ''

def getStockValue(stock, name, supplier):
    changed = False
    connection = sqlite3.connect("stock.db")
    cursor = connection.execute("SELECT name, supplier, in_stock from stock WHERE name = '{}' and supplier = '{}'".format(name, supplier))
    for row in cursor:
        if name in row[0] and supplier in row[1] and stock != row[2]:
            print('Updating data record')
            connection.execute("UPDATE stock set in_stock = {} where name = '{}' and supplier = '{}'".format(stock, name, supplier))
            connection.commit()
            changed = True
    return changed

def getStock(URL):
    page = requests.get(URL)
    if page.status_code == 200:
        try:
            data = json.loads(page.content)
            if 'available' in data:
                if data['available']:
                    return True
        except json.JSONDecodeError as error:
            print('Error decoding json: ' + error.msg)
            print(page.content)
    return False

def getPI400PIMORONI():
    return getStock("https://shop.pimoroni.com/products/raspberry-pi-400-personal-computer-kit.js")

def getPIZERO():
    return getStock("https://shop.pimoroni.com/products/raspberry-pi-zero-essentials-kit.js")

def getPI400HUT():
    return getStock("https://thepihut.com/products/raspberry-pi-400-personal-computer-kit.js")

if __name__ == "__main__":
    topic = get_config()
    if len(topic) > 0:
        stock = getPI400PIMORONI()
        changed = getStockValue(int(stock), "PI 400", "Pimoroni")
        if stock and changed:
            requests.post("https://ntfy.sh/" + topic, data="PI 400 is in Stock at Pimoroni 😀".encode(encoding='utf-8'))
        stock = getPIZERO()
        changed = getStockValue(int(stock), "PI Zero", "Pimoroni")
        if stock and changed:
            requests.post("https://ntfy.sh/" + topic, data="PI Zero is in Stock at Pimoroni 😀".encode(encoding='utf-8'))
        stock = getPI400HUT()
        changed = getStockValue(int(stock), "PI 400", "PI Hut")
        if stock and changed:
            requests.post("https://ntfy.sh/" + topic, data="PI 400 is in Stock at Pi Hut 😀".encode(encoding='utf-8'))
