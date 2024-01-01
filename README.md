# Intelligence X Public SDK

The software development kit (SDK) allows anyone to use the Intelligence X search engine. It is free to use and Intelligence X welcomes any integrations into 3rd party tools and services.

Intelligence X is a search engine and data archive. For additional details please visit <https://intelx.io>.

The SDK contains these parts:

1. [API documentation](Intelligence%20X%20API.pdf)
2. [HTML code example](HTML/search.html)
3. [PHP code example](PHP/index.php)
4. [Python code examples](Python/)
5. [Go package and code](Go/ixapi/README.md)
6. [Maltego Transform](Maltego%20Transform/README.md)

Latest updates:
* 12.04.2020 - Version 2: New Python API wrapper and Command Line Interface
* 24.06.2020 - Version 3: Additional filter for [phonebook.cz](https://phonebook.cz) like lookups in Python code
* 04.07.2020 - Version 4: New Maltego Transform

You will need an API key which you can obtain at https://intelx.io/account?tab=developer. Please note that integration into your commercial service/product requires a paid license. If your product is open source, do not embed your API key. The use of public API keys is discontinued.

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
* Credit Card Number
* Social Security Number
* IBAN

## Contact

We love contributions! Feel free to use the issue tracker for any feature requests, bug reports and contributions. You can contact us via email <info@intelx.io>.

The Terms of Service https://intelx.io/terms-of-service apply.

&copy; 2018 - 2024 Intelligence X
