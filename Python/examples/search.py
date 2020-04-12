from intelxapi import intelx

startdate = "2014-01-01 00:00:00"
enddate = "2015-02-02 23:00:00"

intelx = intelx()
search = intelx.search('riseup.net')

for record in search['records']:
    print(f"Found media type {record['media']} in {record['bucket']}")

