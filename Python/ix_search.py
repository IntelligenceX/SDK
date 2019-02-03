#!/usr/bin

import re,sys,os,time
import requests
import json
from termcolor import colored
import urllib, urllib2


def ix_search(term):
    searchurl = 'https://public.intelx.io/intelligent/search'
    
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/536.5 (KHTML, like Gecko) Chrome/19.0.1084.52 Safari/536.5",
        "x-key": "9df61df0-84f7-4dc7-b34c-8ccfb8646ace",
        "Host": "public.intelx.io",
        "Content-Length": "212"
        }

    payload = {
        "term": term,
        "buckets": [], #default public API is web bucket
        "lookuplevel": 0,
        "maxresults": 10, #change the limit of max results here
        "timeout": 0,
        "datefrom": "",
        "dateto": "",
        "sort": 4,
        "media": 0,
        "terminate": []
    } 

    
    getid = requests.post(searchurl,data=json.dumps(payload),headers=headers)
    id_response = getid.json()

    #Authenticate to API
    if id_response['status'] == 0:
        print colored("[+]Successful API Authentication. Starting records search.","green")
        #Craft API URL with the id to return results
        resulturl = "https://public.intelx.io/intelligent/search/result?id=%s" %str(id_response['id'])

        getresults = requests.get(resulturl,headers=headers)
        #print getresults
        data = getresults.json()
        
        if data['status'] == 0 or data['status'] == 1:
            #print data in json format to manipulate as desired
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
    #print id_response['id']


if __name__ == "__main__":

    start = time.time()
    #python ix_search.py <selector>
    ixsearch(sys.argv[1])
    end = time.time()
    print colored("[*] The script executed in [" + str((end-start)) + " seconds|" + str(((end-start)/60)) + " minutes].","blue")
