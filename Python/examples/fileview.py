from intelxapi import intelx

intelx = intelx()
result = intelx.search('riseup.net')

# grab file contents of first search result
contents = intelx.FILE_VIEW(result['records'][0]['type'], result['records'][0]['media'], result['records'][0]['storageid'], result['records'][0]['bucket'])

print(contents)
