# IntelX

IntelX is a Python command-line utility and API wrapper for intelx.io, made to perform any kind of open-source intelligence.

![](cli/screenshot1.png)

## Installation
```bash
git clone https://github.com/IntelligenceX/SDK
pip3 install SDK/Python
```

## Setup

To specify the API key to use, you can choose one of two options:
* Setting the ```INTELX_KEY``` environment variable.
* Manually supplying the ```-apikey``` argument.

You can get your API key here: https://intelx.io/account?tab=developer

##### Environment Variable
```bash
# create an INTELX_KEY env var with your API key.
export INTELX_KEY=00000000-0000-0000-0000-000000000000
```

##### Via the client

```bash
intelx.py -search riseup.net -apikey 00000000-0000-0000-0000-000000000000
```

## Configuration

On windows, we need to manually configure the command prompt/terminal in order to enable color support. You can do that with the following instructions:

1. Create following file ```Enable Color.reg```
```
Windows Registry Editor Version 5.00
[HKEY_CURRENT_USER\Console]
"VirtualTerminalLevel"=dword:00000001
```

2. Right Click ```Enable Color.reg``` -> Merge

## Usage

```bash
intelx.py -search riseup.net
```


#### Quick search
```bash
intelx.py -search riseup.net
```

#### Quick search in buckets
```bash
intelx.py -search riseup.net -buckets "pastes, darknet.tor"
```

#### Search with 100 results
```bash
intelx.py -search riseup.net -limit 100
```

#### Download Item

The ```-download``` argument will set the HTTP request type to a stream, ultimately returning the raw bytes.
This allows us to download documents such as PDFs, ZIP, Word documents, Excel, etc.
You may set the filename with the ```-name``` argument.
```bash
# save item as test.pdf
intelx.py -download 29a97791-1138-40b3-8cf1-de1764e9d09c -name test.txt
```

#### View Item

To view the full data of a specific search result, specify the item's ID and use the `--view` parameter:

```bash
intelx.py -search 3a4d5699-737c-4d22-8dbd-c5391ce805df --view
```

#### Search Phonebook
```bash
intelx.py -search cia.gov --phonebook
```

#### Extract email from phonebook search
```bash
intelx.py -search cia.gov --phonebook --emails
```

