from intelxapi import intelx

intelx = intelx()
search = intelx.search('riseup.net')

# save the first search result file as "file.contents"
intelx.FILE_READ(search['records'][0]['systemid'], 0, search['records'][0]['bucket'], "file1.bin")