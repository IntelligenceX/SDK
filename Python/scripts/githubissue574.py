# https://github.com/IntelligenceX/SDK/tree/master/Python#searching-in-specific-buckets
import os
from intelxapi import intelx

b = ['leaks.logs','leaks.private.general']

if 'INTELX_KEY' in os.environ:
    ix = intelx(os.environ['INTELX_KEY'])

elif args.apikey:
    ix = intelx(args.apikey)

results = ix.search('hackerone.com', maxresults=5, buckets=b)
