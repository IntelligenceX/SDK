import intelxapi, pathlib, json
from maltego_trx.maltego import UIM_TYPES
from maltego_trx.entities import URL

from maltego_trx.transform import DiscoverableTransform


class ixsearch(DiscoverableTransform):
    @classmethod
    def create_entities(cls, request, response):
        domain_name = request.Value
        try:
            path = pathlib.Path(__file__).parent.absolute()
            with open(f"{path}/../settings.json", 'r') as h:
                contents = h.read().strip('\n')
                settings = json.loads(contents)
                key = settings['APIKEY']
                h.close()
            intelx = intelxapi.intelx(key, ua='IX Maltego Transform/2')
            results = intelx.search(domain_name)
            for record in results['records']:
                if record['name'] == "":
                    name = record['systemid']
                else:
                    # we have to strip all unicode chracters, cuz maltego-trx can not handle them properly
                    name = record['name']
                    stripped_name = (c for c in name if 0 < ord(c) < 127)
                    name = ''.join(stripped_name)

                entity = response.addEntity('intelx.searchresult')
                entity.addProperty('properties.intelligencexsearchresult', 'properties.intelligencexsearchresult', 'loose', name)
                entity.addProperty('SID', 'SID', 'loose', record['systemid'])
                entity.addProperty("Type", "Type", "loose", record['type'])
                entity.addProperty("Media", "Media", "loose", record['media'])
                entity.addProperty("Bucket", "Bucket", "loose", record['bucket'])

        except Exception as e:
            response.addUIMessage("Error: " + str(e), UIM_TYPES["partial"])