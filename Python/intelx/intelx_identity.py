import time
import inspect  # DEBUG
import logging  # DEBUG
import http.client # DEBUG
import requests

from intelxapi import intelx

http.client.HTTPConnection.debuglevel = 1  # DEBUG

logging.basicConfig()  # DEBUG
logging.getLogger().setLevel(logging.DEBUG)  # DEBUG
requests_log = logging.getLogger("requests.packages.urllib3")  # DEBUG
requests_log.setLevel(logging.DEBUG)  # DEBUG
requests_log.propagate = True  # DEBUG

class IdentityService(intelx):

    def __init__(self, api_key, user_agent='IX-Python/0.6'):
        frame = inspect.currentframe()  # DEBUG
        logging.debug('PYthon Function: %s PYthon Filename: %s PYthon Line Number: %s', frame.f_code.co_name, frame.f_code.co_filename, frame.f_lineno)  # DEBUG
        super().__init__(api_key, user_agent)
        self.API_ROOT = 'https://3.intelx.io'
        self.HEADERS = {'X-Key': self.API_KEY, 'User-Agent': self.USER_AGENT}
        self.PAUSE_BETWEEN_REQUESTS = 1

    def get_search_results(self, id, format=1, maxresults=100):
        frame = inspect.currentframe()  # DEBUG
        logging.debug('PYthon Function: %s PYthon Filename: %s PYthon Line Number: %s', frame.f_code.co_name, frame.f_code.co_filename, frame.f_lineno)  # DEBUG
        params = {'id': id, 'format': format, 'limit': maxresults}
        r = requests.get(self.API_ROOT + '/live/search/result',
                         params, headers=self.HEADERS)
        if r.status_code == 200:
            return r.json()
        else:
            return r.status_code

    def idsearch(self, term, maxresults=100, buckets="", timeout=5, datefrom="", dateto="",
               terminate=[], analyze=False, skip_invalid=False):
        frame = inspect.currentframe()  # DEBUG
        logging.debug('PYthon Function: %s PYthon Filename: %s PYthon Line Number: %s', frame.f_code.co_name, frame.f_code.co_filename, frame.f_lineno)  # DEBUG
        p = {
            "selector": term,
            "bucket": buckets,
            "skipinvalid": skip_invalid,
            "limit": maxresults,
            "analyze": analyze,
            "datefrom": datefrom,  # "YYYY-MM-DD HH:MM:SS",
            "dateto": dateto,  # "YYYY-MM-DD HH:MM:SS"
            "terminate": terminate,
        }
        done = False
        results = []
        r = requests.get(self.API_ROOT + '/live/search/internal',
                         headers=self.HEADERS, params=p)
        if r.status_code == 200:
            search_id = r.json()['id']
        else:
            return (r.status_code, r.text)
        if (len(str(search_id)) <= 3):
            print(
                f"[!] intelx.IDENTITY_SEARCH() Received {self.get_error(search_id)}")
        while not done:
            time.sleep(self.PAUSE_BETWEEN_REQUESTS)
            r = self.get_search_results(search_id, maxresults=maxresults)
            if (r["status"] == 0 and r["records"]):
                for a in r['records']:
                    results.append(a)
                maxresults -= len(r['records'])
            if (r['status'] == 2 or maxresults <= 0):
                for a in r['records']:
                    results.append(a)
                if (maxresults <= 0):
                    self.terminate_search(search_id)
                done = True
            if r['status'] == 3:
                self.terminate_search(search_id)
                done = True
        return {'records': results}

    def terminate_search(self, id):
        frame = inspect.currentframe()  # DEBUG
        logging.debug('PYthon Function: %s PYthon Filename: %s PYthon Line Number: %s', frame.f_code.co_name, frame.f_code.co_filename, frame.f_lineno)  # DEBUG
        p = {
            "id": id,
        }
        r = requests.get(self.API_ROOT + '/live/search/internal',
                         headers=self.HEADERS, params=p)
        if r.status_code == 204:
            return (r.status_code, r.text)
        else:
            return (r.status_code, r.text)

    def export_accounts(self, term, datefrom=None, dateto=None, maxresults=10, buckets="", terminate=None):
        frame = inspect.currentframe()  # DEBUG
        logging.debug('PYthon Function: %s PYthon Filename: %s PYthon Line Number: %s', frame.f_code.co_name, frame.f_code.co_filename, frame.f_lineno)  # DEBUG
        p = {
            "selector": term,
            "bucket": buckets,
            "limit": maxresults,
            "datefrom": datefrom,  # "YYYY-MM-DD HH:MM:SS",
            "dateto": dateto,  # "YYYY-MM-DD HH:MM:SS"
            "terminate": terminate,
        }
        done = False
        results = []
        r = requests.get(self.API_ROOT + '/accounts/csv',
                         headers=self.HEADERS, params=p)
        if r.status_code == 200:
            search_id = r.json()['id']
            if (len(str(search_id)) <= 3):
                print(
                    f"[!] intelx.IDENTITY_EXPORT() Received {self.get_error(search_id)}")
            while not done:
                time.sleep(self.PAUSE_BETWEEN_REQUESTS)
                r = self.get_search_results(search_id, maxresults=maxresults)
                if (r["status"] == 0 and r["records"]):
                    for a in r['records']:
                        results.append(a)
                    maxresults -= len(r['records'])
                if (r['status'] == 2 or maxresults <= 0):
                    if (maxresults <= 0):
                        self.terminate_search(search_id)
                    done = True
            return {'records': results}
        else:
            return (r.status_code, r.text)
