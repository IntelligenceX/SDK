import intelxapi, pathlib, json, html
from maltego_trx.maltego import UIM_TYPES
from maltego_trx.entities import Domain, URL, Email, IPAddress, PhoneNumber

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
            intelx = intelxapi.intelx(key, ua='IX Maltego Transform/3')
            results = intelx.search(domain_name, maxresults=12) # request.Slider() returns 100 in Maltego CE, temp hardcode to 12 for testing
            for record in results['records']:
                if record['name'] == "":
                    name = record['systemid']
                else:
                    # we have to strip all unicode chracters, cuz maltego-trx can not handle them properly
                    name = record['name']
                    stripped_name = (c for c in name if 0 < ord(c) < 127)
                    name = ''.join(stripped_name)

                if record['mediah'] == "Domain": # Domain
                    entity = response.addEntity(Domain, record['name'])

                elif len(record['historyfile']) > 0:
                    entity = response.addEntity('intelx.historicalsearchresult', name)
                    entity.addProperty('properties.intelligencexhistoricalsearchresult', 'properties.intelligencexhistoricalsearchresult', 'loose', name)
                    entity.addProperty("Historyfile", "Historyfile", "loose", record['historyfile'])
                    entity.addProperty("Indexfile", "Indexfile", "loose", record['indexfile'])
                    preview = intelx.FILE_PREVIEW(record['type'], record['media'], 0, record['storageid'], record['bucket'])
                    preview = preview.replace('\n', '<br>')
                    preview = html.escape(preview)
                    stripped_preview = (c for c in preview if 0 < ord(c) < 127)
                    preview = ''.join(stripped_preview)
                    entity.addDisplayInformation(preview, 'Preview')

                elif 'leak' in record['bucket']:
                    entity = response.addEntity('intelx.leak', name)
                    entity.addProperty('properties.intelligencexleak', 'properties.intelligencexleak', 'loose', name)
                    entity.addProperty("Indexfile", "Indexfile", "loose", record['indexfile'])
                    preview = intelx.FILE_PREVIEW(record['type'], record['media'], 0, record['storageid'], record['bucket'])
                    preview = preview.replace('\n', '<br>')
                    preview = html.escape(preview)
                    stripped_preview = (c for c in preview if 0 < ord(c) < 127)
                    preview = ''.join(stripped_preview)
                    entity.addDisplayInformation(preview, 'Preview')

                else:
                    entity = response.addEntity('intelx.searchresult', name)
                    entity.addProperty('properties.intelligencexsearchresult', 'properties.intelligencexsearchresult', 'loose', name)
                    preview = intelx.FILE_PREVIEW(record['type'], record['media'], 0, record['storageid'], record['bucket'])
                    preview = preview.replace('\n', '<br>')
                    preview = html.escape(preview)
                    stripped_preview = (c for c in preview if 0 < ord(c) < 127)
                    preview = ''.join(stripped_preview)
                    entity.addDisplayInformation(preview, 'Preview')

                entity.addProperty('SID', 'SID', 'loose', record['systemid'])
                entity.addProperty('STORAGEID', 'STORAGEID', 'loose', record['storageid'])
                entity.addProperty("Type", "Type", "loose", record['type'])
                entity.addProperty("Media", "Media", "loose", record['media'])
                entity.addProperty("Bucket", "Bucket", "loose", record['bucket'])

        except Exception as e:
            response.addUIMessage("Error: " + str(e), UIM_TYPES["partial"])
