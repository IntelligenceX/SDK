# Introduction

`httpie` is available from [httpie.io](https://httpie.io/docs/cli/installation).

# Identity Portal

In the following examples `11111111-1111-1111-1111-111111111111` represents the @IntelligenceX Key and `00000000-0000-0000-0000-000000000000` represents the @IntelligenceX `Search ID`.

## -identityenabled
```
$ http "https://3.intelx.io/live/search/internal" selector==example.com skipinvalid==False limit==10 analyze==False datefrom== dateto== "x-key:11111111-1111-1111-1111-111111111111"
HTTP/1.1 200 OK
Access-Control-Allow-Origin: *
Cache-Control: no-cache, no-store, must-revalidate
Content-Length: 57
Content-Type: application/json
Date: Wed, 06 Mar 2024 02:02:02 GMT
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload

{
    "id": "00000000-0000-0000-0000-000000000000",
    "status": 0
}


                                                                                                                                                                                                                                           
$ http "https://3.intelx.io/live/search/result" id==00000000-0000-0000-0000-000000000000 format==1 limit==10 "x-key:11111111-1111-1111-1111-111111111111" --download -o pyintelx-identityenabled-example.com.json 
HTTP/1.1 200 OK
Access-Control-Allow-Origin: *
Cache-Control: no-cache, no-store, must-revalidate
Content-Type: application/json
Date: Wed, 06 Mar 2024 02:02:30 GMT
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
Transfer-Encoding: chunked

Downloading to pyintelx-identityenabled-example.com.json
Done. 1.1 MB in 00:10.10101 (101.1 kB/s)
                                                                                                                                                                                                                                           
$
```
## -identityenabled -accounts
```
$ http "https://3.intelx.io/accounts/csv" selector==example.com limit==10 datefrom== dateto== "x-key:11111111-1111-1111-1111-111111111111"
HTTP/1.1 200 OK
Access-Control-Allow-Origin: *
Cache-Control: no-cache, no-store, must-revalidate
Content-Length: 57
Content-Type: application/json
Date: Wed, 06 Mar 2024 01:01:01 GMT
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload

{
    "id": "00000000-0000-0000-0000-000000000000",
    "status": 0
}



$ http "https://3.intelx.io/live/search/result" id==00000000-0000-0000-0000-000000000000 "x-key:11111111-1111-1111-1111-111111111111" --download -o pyintelx-identityenabled-accounts-example.com.json
HTTP/1.1 200 OK
Access-Control-Allow-Origin: *
Cache-Control: no-cache, no-store, must-revalidate
Content-Length: 49
Content-Type: application/json
Date: Wed, 06 Mar 2024 01:01:31 GMT
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload

Downloading to pyintelx-identityenabled-accounts-example.com.json
Done. 100 bytes in 00:0.10000 (100.00000000000000 bytes/s)
                                                                                                                                                                                                                                           
$
