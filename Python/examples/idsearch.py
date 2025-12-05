from intelx import IdentityService
from dotenv import load_dotenv
import os
import json

load_dotenv()
INTELX_KEY = os.getenv("INTELX_KEY")

intelx = IdentityService(INTELX_KEY)
result = intelx.idsearch('john.doe@example.com')

print(f'Found {len(result['records'])} records like...')

print(json.dumps(result['records'][0], indent=2, ensure_ascii=False))