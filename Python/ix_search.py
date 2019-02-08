#!/usr/bin

import re,sys,os,time
import requests
import json
from termcolor import colored
import urllib, urllib2


def ix_search(baseurl,term):

    print baseurl
    print apikey
    print term

    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/536.5 (KHTML, like Gecko) Chrome/19.0.1084.52 Safari/536.5",
        "x-key": '9df61df0-84f7-4dc7-b34c-8ccfb8646ace',
        "Host": baseurl,
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

    searchurl = 'https://' + baseurl + '/intelligent/search'
    print searchurl
    getid = requests.post("https://public.intelx.io/intelligent/search",data=json.dumps(payload),headers=headers)

    if getid.status_code == 200:
        id_response = getid.json()

        #Authenticate to API
        if id_response['status'] == 0:
            print colored("[+]Successful API Authentication. Starting records search.","green")
            #Craft API URL with the id to return results
            resulturl = str(searchurl) + "/result?id=%s" %str(id_response['id'])

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

    else:
        print "----------------------------------------------"
        print "[!] Error Code Status: <" + str(getid.status_code) + ">"
        print "----------------------------------------------"
        print "204 | No Content"
        print "400 | Bad Request. Invalid input."
        print "401 | Unauthorized. Access not authorized."
        print "402 | Payment Required. No credits available"
        print "404 | Not Found. Item or identifier not found."
        print "----------------------------------------------"

if __name__ == "__main__":
    #https://public.intelx.io
    #9df61df0-84f7-4dc7-b34c-8ccfb8646ace
    #python ix_search.py <baseurl> <apikey> <selector>
    #python ix_search.py https://public.intelx.io 9df61df0-84f7-4dc7-b34c-8ccfb8646ace test.com
    ix_search(sys.argv[1],sys.argv[2],sys.argv[3])

