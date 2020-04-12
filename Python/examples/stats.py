from intelxapi import intelx

intelx = intelx()

search = intelx.search(
  'riseup.net',
  maxresults=1000,
)

stats = intelx.stats(search)
print(stats)