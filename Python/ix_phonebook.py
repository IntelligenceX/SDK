import re,sys,os,time
import requests
import json
from termcolor import colored
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

    print(colored("[+] " + str(date.today()) + ": Kicking off phonebook query of " + str(term) + " with max results set at " + str(payload['maxresults']),"red"))

    #initial POST to intelligence X service to retrieve the results ID and do all the fun stuff
    getid = requests.post("https://" + baseurl + "/phonebook/search",data=json.dumps(payload),headers=headers)
    id_response = getid.json()
    
    if id_response['status'] == 0:
        print(colored("[+]Successful API Authentication. Searching available records...","green"))
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

    start = time.time()
    #python ix_phonebook.py <selector>
    ixphonebook("public.intelx.io","9df61df0-84f7-4dc7-b34c-8ccfb8646ace",sys.argv[1])
    end = time.time()
    print(colored("[*] The script executed in [" + str((end-start)) + " seconds|" + str(((end-start)/60)) + " minutes].","blue"))
