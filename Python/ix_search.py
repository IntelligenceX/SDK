import re,sys,os,time
import requests
import json
import urllib


def ix_search(baseurl,apikey,term):
    
    headers = {
        "User-Agent": "ix-client/python",
        "x-key": apikey,
        }

    payload = {
        "term": term,
        "buckets": [],
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
    getid = requests.post("https://" + baseurl + "/intelligent/search",data=json.dumps(payload),headers=headers)

    if getid.status_code == 200:
        id_response = getid.json()

        #Authenticate to API
        if id_response['status'] == 0:
            print("[+]Successful API Authentication. Starting records search.")
            #Craft API URL with the id to return results
            resulturl = str(searchurl) + "/result?id=%s" %str(id_response['id'])

            status = 3  # status 3 = No results yet, keep trying. 0 = Success with results
            while status == 3 or status == 0:

                getresults = requests.get(resulturl,headers=headers)
                data = getresults.json()
                status = data['status']
                
                if status == 0 or status == 1:
                    #print data in json format to manipulate as desired
                    print(data)
                elif status == 2:
                    print("----------------------------------------------")
                    print("[!] Error Code 2 Search ID Not Found ")
                    print("----------------------------------------------")

    else:
        print("----------------------------------------------")
        print("[!] Error Code Status: <" + str(getid.status_code) + ">")
        print("----------------------------------------------")
        print("204 | No Content")
        print("400 | Bad Request. Invalid input.")
        print("401 | Unauthorized. Access not authorized.")
        print("402 | Payment Required. No credits available")
        print("404 | Not Found. Item or identifier not found.")
        print("----------------------------------------------")

if __name__ == "__main__":
    #python ix_search.py <selector>
    ix_search("public.intelx.io","9df61df0-84f7-4dc7-b34c-8ccfb8646ace",sys.argv[1])

