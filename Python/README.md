# Intelligence X API Python client

1. [ix_search.py to search and show results](ix_search.py)
2. [ix_phonebook.py to make a phonebook lookup](ix_phonebook.py)

## Usage

First, make sure to download the dependencies:

```
pip install termcolor
pip install requests
```

Using the Python files is straight forward:

```shell
ix_search.py <selector>
ix_phonebook.py <selector>
```

Examples:

```shell
ix_search.py test.com
ix_phonebook.py test.com
```

The results are currently displayed as raw JSON data. Contributions for a more user-friendly representation are welcome.

## Notes

If you have an account on intelx.io go to the developer tab at https://intelx.io/account?tab=developer to get your own API key and URL.

In the `ix_search.py` change the default API host and key here:

```python
ix_search("public.intelx.io","9df61df0-84f7-4dc7-b34c-8ccfb8646ace",sys.argv[1])
```

The same for `ix_phonebook.py`:

```python
ixphonebook("public.intelx.io","9df61df0-84f7-4dc7-b34c-8ccfb8646ace",sys.argv[1])
```
