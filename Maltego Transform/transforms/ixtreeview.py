import intelxapi, pathlib, json
from maltego_trx.maltego import UIM_TYPES
from maltego_trx.entities import URL

from maltego_trx.transform import DiscoverableTransform

import traceback


class ixtreeview(DiscoverableTransform):
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
            history = intelx.treeview(str(request.getProperty("Indexfile")), str(request.getProperty("Bucket")))
            for entry in history:
                entity = response.addEntity('intelx.searchresult', entry['name'])
                entity.addProperty('SID', 'SID', 'loose', entry['systemid'])
                entity.addProperty("Type", "Type", "loose", entry['type'])
                entity.addProperty("Media", "Media", "loose", entry['media'])
                entity.addProperty("Bucket", "Bucket", "loose", entry['bucket'])

        except Exception as e:
            response.addUIMessage("Error: " + str(e), UIM_TYPES["partial"])
