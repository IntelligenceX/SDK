import intelxapi, pathlib, json
from maltego_trx.maltego import UIM_TYPES
from maltego_trx.entities import URL

from maltego_trx.transform import DiscoverableTransform


class ixurls(DiscoverableTransform):
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
            results = intelx.phonebooksearch(domain_name, target=3)
            for selector in results:
                for result in selector['selectors']:
                    entity = response.addEntity(URL)
                    entity.addProperty('short-title', 'Title', 'loose', result['selectorvalue'])
                    entity.addProperty('url', 'URL', 'loose', result['selectorvalue'])

        except Exception as e:
            response.addUIMessage("Error: " + str(e), UIM_TYPES["partial"])