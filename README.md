# Intelligence X Public SDK

The software development kit (SDK) allows anyone to use the Intelligence X search engine. It is free to use and Intelligence X welcomes any integrations into 3rd party tools and services.

Intelligence X is a search engine and data archive. For additional details please visit <https://intelx.io>.

The SDK contains these parts:

1. [API documentation](Intelligence%20X%20API.pdf)
2. [HTML code example](HTML/search.html)
3. [PHP code example](PHP/index.php)
4. [Python code examples](Python/)
5. [Go package and code](Go/ixapi/README.md)
6. [API calls in Fiddler archive](Fiddler%20Examples.saz)

## Link to intelx.io

Instead of directly using the API, you can always do the ghetto version instead and just link to the website.

```
https://intelx.io/?s=[search term]
```

Examples:

```
https://intelx.io/?s=test.com
https://intelx.io/?s=test@example.com
```

The search engine supports only the following strong selector types. Anything else will be rejected.

* Email address
* Domain, including wildcards like *.example.com
* URL
* IPv4 and IPv6
* CIDRv4 and CIDRv6
* Phone Number
* Bitcoin address
* MAC address
* IPFS Hash
* UUID
* Simhash
* Credit card number
* IBAN

## Credits

Special thanks for their contributions to:
* Cyberblack for providing the Python code in Q1 2019

## Contact

We love contributions! Feel free to use the issue tracker for any feature requests, bug reports and contributions. You can contact us via email <info@intelx.io>.

&copy; 2018 - 2019 Intelligence X
