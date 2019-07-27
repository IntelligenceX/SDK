# Intelligence X API Python client

1. [ix_search.py to search and show results](ix_search.py)
2. [ix_phonebook.py to make a phonebook lookup](ix_phonebook.py)

## Usage

First, make sure to download the dependencies:

```
pip install requests
```

Using the Python files is straight forward:

```shell
ix_search.py <api domain> <api key> <search selector>
ix_phonebook.py <api domain> <api key> <search selector>
```

Examples:

```shell
ix_search.py public.intelx.io 9df61df0-84f7-4dc7-b34c-8ccfb8646ace test.com
ix_phonebook.py public.intelx.io 9df61df0-84f7-4dc7-b34c-8ccfb8646ace test.com
```

The results are currently displayed as raw JSON data. Contributions for a more user-friendly representation are welcome.

## API Details

To use the API you first need an account on intelx.io. Go to the developer tab at https://intelx.io/account?tab=developer to get your own API key and domain.
