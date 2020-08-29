import intelxapi, pathlib, json
from maltego_trx.maltego import UIM_TYPES
from maltego_trx.entities import Domain

from maltego_trx.transform import DiscoverableTransform


class ixsubdomains(DiscoverableTransform):
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
            results = intelx.phonebooksearch(domain_name, target=1)
            for selector in results:
                for result in selector['selectors']:
                    response.addEntity(Domain, result['selectorvalue'])

        except Exception as e:
            response.addUIMessage("Error: " + str(e), UIM_TYPES["partial"])