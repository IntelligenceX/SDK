#!/usr/bin/env python3
# pylint: disable-msg=E0611

import os
import sys
import html
import json
import time
import tabulate
import argparse
from intelxapi import intelx
from intelx_identity import IdentityService
from termcolor import colored
from pygments import highlight
from pygments.lexers import JsonLexer
from pygments.formatters import TerminalFormatter

BOLD = '\033[1m'
END = '\033[0m'

banner = r'''
{}     _____      _       ___   __
    |_   _|    | |     | \ \ / /
      | | _ __ | |_ ___| |\ V /
      | || '_ \| __/ _ \ |/   \
     _| || | | | ||  __/ / /^\ \
     \___/_| |_|\__\___|_\/   \/

       a command line client
           for intelx.io         {}

'''.format(BOLD, END)


def rightnow():
    return time.strftime("%H:%M:%S")


def search(ix, query, maxresults=100, buckets=[], timeout=5, datefrom="", dateto="", sort=4, media=0, terminate=[]):
    if not args.raw:
        print(colored(f"[{rightnow()}] Starting search of \"{args.search}\".", 'green'))
    s = ix.search(args.search, maxresults, buckets, timeout, datefrom, dateto, sort, media, terminate)
    return s


def pbsearch(ix, query, maxresults=100, buckets=[], timeout=5, datefrom="", dateto="", sort=4, media=0, terminate=[], target=0):
    if not args.raw:
        print(colored(f"[{rightnow()}] Starting phonebook search of \"{args.search}\".", 'green'))
    s = ix.phonebooksearch(args.search, maxresults, buckets, timeout, datefrom, dateto, sort, media, terminate, target)
    return s


def get_stats(stats):
    if not args.raw:
        print(colored(f"[{rightnow()}] Gathering stats from search.\n", 'green'))
        stats = json.dumps(ix.stats(search), indent=4, sort_keys=True)
        print(stats)


def format_list(content):
    content = content.replace(" ", "")
    return content.split(",")


def quick_search_results(ix, search, limit):
    for idx, result in enumerate(search['records']):
        if(idx == limit):
            sys.exit()
        else:
            if args.view:
                viewtext = ix.FILE_VIEW(result['type'], result['media'], result['storageid'], result['bucket'])
            elif not args.nopreview:
                viewtext = ix.FILE_PREVIEW(result['type'], result['media'], 0, result['storageid'], result['bucket'])
            if(len(result['name']) == 0):
                result['name'] = "Untitled Document"
            print(f"{BOLD}________________________________________________________________________________{END}")
            print(f"{BOLD}> Name:{END}", html.unescape(result['name']))
            print(f"{BOLD}> Date:{END}", result['date'])
            print(f"{BOLD}> Size:{END}", result['size'], "bytes")
            print(f"{BOLD}> Media:{END}", result['mediah'])
            print(f"{BOLD}> Bucket:{END}", result['bucketh'])
            print(f"{BOLD}> ID:{END}", result['systemid'])
            if len(viewtext) > 0:
                print("")
                print(viewtext)
            print(f"{BOLD}________________________________________________________________________________{END}")


def pb_search_results(ix, search):
    headers = ["Type", "Value"]
    data = []
    for block in search:
        for result in block['selectors']:
            data.append([result['selectortypeh'], result['selectorvalue']])
    print(tabulate.tabulate(sorted(data), headers=headers, tablefmt="fancy_grid"))


def pb_search_results_emails(ix, search):
    for block in search:
        for result in block['selectors']:
            if result['selectortype'] == 1:
                print(result['selectorvalue'])

def idsearch(identity_ix, query, maxresults=100, buckets=[], timeout=5, datefrom="", dateto="", terminate=[]):
    if not args.raw:
        print(colored(f"[{rightnow()}] Starting search of \"{args.search}\".", 'green'))
    s = identity_ix.search(term=query, maxresults=maxresults, buckets=buckets, timeout=timeout, datefrom=datefrom, dateto=dateto,  terminate=terminate)
    return s

def main(argv=None):

    global search
    global accounts
    global args

    # get the argument parser ready
    parser = argparse.ArgumentParser(
        description="Command line interface for https://intelx.io",
        epilog="Usage: intelx -search 'riseup.net' -buckets 'pastes, darknet'"
    )

    parser.add_argument('-apikey', help="set the api key via command line")
    parser.add_argument('-search', help="search query")
    parser.add_argument('-identity', help="search only accounts on identity service")
    parser.add_argument('-buckets', help="set which buckets to search")
    parser.add_argument('-limit', help="set the amount of results to show")
    parser.add_argument('-timeout', help="set the timeout value")
    parser.add_argument('-datefrom', help="begin search starting from state")
    parser.add_argument('-dateto', help="begin search ending from date")
    parser.add_argument('-sort', help="set the sort value")
    parser.add_argument('-media', help="set the media value")
    parser.add_argument('-lines', help="set the number of lines displayed in the preview")
    parser.add_argument('-download', help="download the specified item specified by its ID")
    parser.add_argument('-bucket', help="download from this bucket (must be specified with -download)")
    parser.add_argument('-name', help="set the filename to save the item as")
    parser.add_argument('--dataleaks', help="searches for a domain or email address to find data leaks", action="store_true")
    parser.add_argument('--exportaccounts', help="searches for a domain or email address to find leaked accounts.", action="store_true")
    parser.add_argument('--nopreview', help="do not show text preview snippets of search results", action="store_true")
    parser.add_argument('--view', help="show full contents of search results", action="store_true")
    parser.add_argument('--phonebook', help="set the search type to a phonebook search")
    parser.add_argument('--emails', help="show only emails from phonebook results", action="store_true")
    parser.add_argument('--capabilities', help="show your account's capabilities", action="store_true")
    parser.add_argument('--stats', help="show stats of search results", action="store_true")
    parser.add_argument('--raw', help="show raw json", action="store_true")
    args = parser.parse_args(argv)

    # configure IX & the API key
    if 'INTELX_KEY' in os.environ:
        if args.identity:
            ix = IdentityService(os.environ['INTELX_KEY'])
        else:
            ix = intelx(os.environ['INTELX_KEY'])

    elif args.apikey:
        if args.identity:
            ix_identity = IdentityService(args.apikey)
        else:
            ix = intelx(args.apikey)

    else:
        exit('No API key specified. Please use the "-apikey" parameter or set the environment variable "INTELX_KEY".')

    # main application flow
    if not args.raw:
        print(banner)

    if len(sys.argv) < 2:
        print('Usage: intelx -search "riseup.net"')

    if args.identity:

        if not args.limit and not args.stats and not args.phonebook:
            if not args.raw:
                print(colored(f"[{rightnow()}] Limit argument not supplied, setting default to 10 results.", 'yellow'))
            args.limit = 10

        maxresults = 100
        buckets = []
        datefrom = ""
        dateto = ""
        sort = 4
        media = 0
        terminate = []

        if args.limit:
            maxresults = int(args.limit)
        if args.buckets:
            buckets = format_list(args.buckets)
        if args.datefrom:
            datefrom = args.datefrom
        if args.dateto:
            dateto = args.dateto
            sort = 2  # sort by date
        if args.sort:
            sort = int(args.sort)
        if args.media:
            media = int(args.media)

        if args.exportaccounts:
            print(colored(f"[{rightnow()}] Starting account export of \"{args.identity}\".", 'green'))
            account = IdentityService.export_accounts(
                    ix,
                    args.identity,
                    maxresults=maxresults,
                    buckets=buckets,
                    datefrom=datefrom,
                    dateto=dateto,
                    terminate=terminate
            )
            headers = ["User", "Password", "Password Type", "Source Short"]
            data = []
            for block in account:
                for result in account[block]:
                    data.append([result['user'], result['password'], result['passwordtype'], result['sourceshort']])
            print(tabulate.tabulate(sorted(data), headers=headers, tablefmt="fancy_grid"))
            exporttsv=tabulate.tabulate(data, tablefmt="tsv")
            tsv_filename = "intelx-output-" + args.identity + "-export_accounts.tsv"
            tsv_file=open(tsv_filename,"w")
            tsv_file.write(exporttsv)
            tsv_file.close()
            print(colored(f"[{rightnow()}] Exported output to \"{tsv_filename}\".", 'green'))

        if args.dataleaks:
                print(colored(f"[{rightnow()}] Starting data leaks search of \"{args.identity}\".", 'green'))
                search = IdentityService.idsearch(
                    ix,
                    args.identity,
                    maxresults=maxresults,
                    buckets=buckets,
                    datefrom=datefrom,
                    dateto=dateto,
                    terminate=terminate
                )
                headers = ["Name", "Date", "Bucket", "Line"]
                data = []
                for records in search:
                    for result in search[records]:
                        data.append([result['item']['name'], result['item']['date'], result['item']['bucket'], result['linea']])
                exporttsv=tabulate.tabulate(data, tablefmt="tsv")
                tsv_filename = "intelx-output-" + args.identity + "-data_leaks.tsv"
                tsv_file=open(tsv_filename,"w")
                tsv_file.write(exporttsv)
                tsv_file.close()
                print(colored(f"[{rightnow()}] Exported output to \"{tsv_filename}\".", 'green'))

    if args.search:

        if not args.limit and not args.stats and not args.phonebook:
            if not args.raw:
                print(colored(f"[{rightnow()}] Limit argument not supplied, setting default to 10 results.", 'yellow'))
            args.limit = 10

        maxresults = 100
        buckets = []
        timeout = 5
        datefrom = ""
        dateto = ""
        sort = 4
        media = 0
        terminate = []

        if args.limit:
            maxresults = int(args.limit)
        if args.buckets:
            buckets = format_list(args.buckets)
        if args.timeout:
            timeout = int(args.timeout)
        if args.datefrom:
            datefrom = args.datefrom
        if args.dateto:
            dateto = args.dateto
            sort = 2  # sort by date
        if args.sort:
            sort = int(args.sort)
        if args.media:
            media = int(args.media)

        if not args.phonebook:
            search = search(
                ix,
                args.search,
                maxresults=maxresults,
                buckets=buckets,
                timeout=timeout,
                datefrom=datefrom,
                dateto=dateto,
                sort=sort,
                media=media,
                terminate=terminate
            )

        elif args.phonebook:
            if(args.phonebook == 'domains'):
                targetval = 1
            elif(args.phonebook == 'emails'):
                targetval = 2
            elif(args.phonebook == 'urls'):
                targetval = 3
            else:
                targetval = 0

            search = pbsearch(
                ix,
                args.search,
                maxresults=maxresults,
                buckets=buckets,
                timeout=timeout,
                datefrom=datefrom,
                dateto=dateto,
                sort=sort,
                media=media,
                terminate=terminate,
                target=targetval
            )

        if args.raw:
            print(json.dumps(search))

        if args.stats:
            get_stats(search)

        elif not args.raw and not args.phonebook:
            quick_search_results(ix, search, int(args.limit))

        elif not args.raw and args.phonebook:
            if args.emails:
                print()
                pb_search_results_emails(ix, search)
            else:
                print()
                pb_search_results(ix, search)

    if args.download:
        if not args.bucket:
            print(colored(f"[{rightnow()}] Failed to download item {args.download} missing bucket name.\n", 'red'))
        else:
            fname = args.download + ".bin"
            if args.name:
                fname = args.name
            if(ix.FILE_READ(args.download, bucket=args.bucket, filename=fname)):
                print(colored(f"[{rightnow()}] Successfully downloaded the file '{fname}'.\n", 'green'))
            else:
                print(colored(f"[{rightnow()}] Failed to download item {args.download}.\n", 'red'))

    if args.capabilities:
        print(colored(f"[{rightnow()}] Getting your API capabilities.\n", 'green'))
        capabilities = ix.GET_CAPABILITIES()
        print(highlight(json.dumps(capabilities, indent=4), JsonLexer(), TerminalFormatter()))

if __name__ == "__main__":
  sys.exit(main())
