import intelxapi, webbrowser, pathlib, json
from maltego_trx.maltego import UIM_TYPES
from maltego_trx.entities import Domain, URL, Email, IPAddress, PhoneNumber
from maltego_trx.transform import DiscoverableTransform

class ixselectors(DiscoverableTransform):
    @classmethod
    def create_entities(cls, request, response):

        try:

            path = pathlib.Path(__file__).parent.absolute()
            sid = request.getProperty("SID")

            with open(f"{path}/../settings.json", 'r') as h:
                contents = h.read().strip('\n')
                settings = json.loads(contents)
                key = settings['APIKEY']
                h.close()

            intelx = intelxapi.intelx(key, ua='IX Maltego Transform/3')
            selectors = intelx.selectors(sid)

            for selector in selectors:
                
                if selector['type'] == 1: # Email
                    entity = response.addEntity(Email, selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])

                elif selector['type'] == 2: # Domain
                    entity = response.addEntity(Domain, selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])

                elif selector['type'] == 3: # URL
                    entity = response.addEntity(URL, selector['selector'])
                    entity.addProperty('url', 'url', 'loose', selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])

                elif selector['type'] == 4: # Phone
                    entity = response.addEntity(PhoneNumber, selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])

                elif selector['type'] == 6: # IP
                    entity = response.addEntity(IPAddress, selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])

                elif selector['type'] == 17: # Credit card
                    entity = response.addEntity('intelx.creditcard', selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])

                elif selector['type'] == 22: # MAC address
                    entity = response.addEntity('intelx.macaddress', selector['selector'])
                    entity.addProperty('MAC Address', 'MAC Address', 'loose', selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])

                else:
                    entity = response.addEntity('intelx.selector', selector['selector'])
                    entity.addProperty('SID', 'SID', 'loose', selector['systemid'])
                    
        except Exception as e:
            response.addUIMessage("Error: " + str(e), UIM_TYPES["partial"])
