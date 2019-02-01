#!/usr/bin

import re,sys,os,time
import requests
from requests.exceptions import HTTPError
import json
from termcolor import colored
import urllib, urllib2


def ix_search(term):
    searchurl = 'https://public.intelx.io/intelligent/search'
    
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/536.5 (KHTML, like Gecko) Chrome/19.0.1084.52 Safari/536.5",
        "x-key": "9df61df0-84f7-4dc7-b34c-8ccfb8646ace",
        "Host": "2.intelx.io",
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
    
