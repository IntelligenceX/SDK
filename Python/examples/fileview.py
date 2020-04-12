from intelxapi import intelx

intelx = intelx()
result = intelx.search('riseup.net')

# grab file contents of first search result
contents = intelx.FILE_VIEW(result['type'], result['media'], result['storageid'], result['bucket'])

print(contents)