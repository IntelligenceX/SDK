import intelxapi, webbrowser, pathlib
from maltego_trx.maltego import UIM_TYPES

from maltego_trx.transform import DiscoverableTransform

class ixsearchresult(DiscoverableTransform):
    @classmethod
    def create_entities(cls, request, response):
        domain_name = request.Value
        try:
            path = pathlib.Path(__file__).parent.absolute()
            sid = request.getProperty("SID")
            target = f'https://intelx.io/?did={sid}'
            webbrowser.open_new(target)
            
        except Exception as e:
            response.addUIMessage("Error: " + str(e), UIM_TYPES["partial"])