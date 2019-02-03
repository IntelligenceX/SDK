#!/usr/bin

import re,sys,os,time
import requests
import json
from termcolor import colored
import urllib, urllib2
from datetime import date
import datetime


reload(sys)  
sys.setdefaultencoding('utf8')
today = str(date.today())

#Phonebook search that will return results for a given selector
def ixphonebook(term):
    searchurl = 'https://2.intelx.io/phonebook/search'
    key = '9dbc5564-00d4-40b6-8d56-dde235b4312c'
    
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/536.5 (KHTML, like Gecko) Chrome/19.0.1084.52 Safari/536.5",
        "x-key": key,
        "Host": "2.intelx.io",
        "Content-Length": "212"
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

    print colored("[+] " + str(today) + ": Kicking off phonebook query of " + str(term) + " with max results set at " + str(payload['maxresults']),"red")

    #initial POST to intelligence X service to retrieve the results ID and do all the fun stuff
    getid = requests.post(searchurl,data=json.dumps(payload),headers=headers)
    id_response = getid.json()
    
    if id_response['status'] == 0:
        print colored("[+]Successful API Authentication. Searching available records...","green")
        resulturl = "https://2.intelx.io/phonebook/search/result?id=%s" %str(id_response['id'])
        
        #return status code 
        getresults = requests.get(resulturl,headers=headers)
        data = getresults.json()
       
        if data['status'] == 0 or data['status'] == 1: 
            print data
            
        else:
            print "----------------------------------------------"
            print "[!] Error Code Status: <" + str(data['status']) + ">"
            print "----------------------------------------------"
            print "Code <2> | Search ID Not Found."
            print "Code <3> | No Results at this time. Try later."
            print "----------------------------------------------"

    if id_response['status'] == 1:
        print "[!]Invalid term used."

    if id_response['status'] == 2:
        print "[!] Reached the MAX number of concurrent connection for this API Key."

if __name__ == "__main__":

    start = time.time()
    #python ix_phonebook.py <selector>
    ixphonebook(sys.argv[1])
    end = time.time()
    print colored("[*] The script executed in [" + str((end-start)) + " seconds|" + str(((end-start)/60)) + " minutes].","blue")
