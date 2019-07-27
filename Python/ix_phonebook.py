import re,sys,os,time
import requests
import json
import urllib
from datetime import date
import datetime


#Phonebook search that will return results for a given selector
def ixphonebook(baseurl,apikey,term):
    
    headers = {
        "User-Agent": "ix-client/python",
        "x-key": apikey,
        }

    payload = {
        "term": term,
        "buckets": [],
        "lookuplevel": 0,
        "maxresults": 10,
        "timeout": 0,
        "datefrom": "",
        "dateto": "",
        "sort": 4,
        "media": 0,
        "terminate": []
    }   

    print("[+] " + str(date.today()) + ": Kicking off phonebook query of " + str(term) + " with max results set at " + str(payload['maxresults']))

    #initial POST to intelligence X service to retrieve the results ID and do all the fun stuff
    getid = requests.post("https://" + baseurl + "/phonebook/search",data=json.dumps(payload),headers=headers)
    id_response = getid.json()
    
    if id_response['status'] == 0:
        print("[+] Successful API Authentication. Searching available records...")
        resulturl = "https://" + baseurl + "/phonebook/search/result?id=%s" %str(id_response['id'])
        
        status = 3  # status 3 = No results yet, keep trying. 0 = Success with results
        while status == 3 or status == 0:

            getresults = requests.get(resulturl,headers=headers)
            data = getresults.json()
            status = data['status']
        
            if status == 0 or status == 1: 
                print(data)
            elif status == 2:
                print("----------------------------------------------")
                print("[!] Error Code 2 Search ID Not Found ")
                print("----------------------------------------------")

    if id_response['status'] == 1:
        print("[!]Invalid term used.")

    if id_response['status'] == 2:
        print("[!] Reached the MAX number of concurrent connections for this API Key.")

if __name__ == "__main__":
    try:
        api_domain = sys.argv[1]
        api_key = sys.argv[2]
        selector = sys.argv[3]
    except IndexError:
        print('usage: python3 ix_phonebook.py <api domain> <api key> <search selector>')
        sys.exit(0)

    if not validators.domain(api_domain):
        print('[error] invalid API domain provided; exiting ...')
        sys.exit(1)

    ixphonebook(api_domain, api_key, selector)
