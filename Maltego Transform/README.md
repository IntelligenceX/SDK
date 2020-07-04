# intelx-maltego

Maltego Transforms for Intelligence X (intelx.io). The following transforms and entities will be installed.

#### Transforms

* Intelligence X Emails Transform
* Intelligence X Search Transform
* Intelligence X URLs Transform
* Intelligence X Subdomains Transform
* Intelligence X Search Result Transform

#### Entities

* Intelligence X BTC Entity
* Intelligence X Credit Card Entity
* Intelligence X IBAN Entity
* Intelligence X MAC Address Entity
* Intelligence X Simhash Entity
* Intelligence X Storage ID Entity
* Intelligence X System ID Entity
* Intelligence X UUID Entity

## Manual Installation

This transform is currently not in the Transform Hub marketplace. Follow the instructions below to install it manually.

**NOTES**: 

* The config is required for the transform after installation. Do not delete it at any point except during uninstallation.
* You *cannot* include any whitespaces in any of the filepaths unless it's for the Python executable.

### Requirements

* [intelx-0.4](https://github.com/IntelligenceX/SDK/tree/master/Python) (included in the instructions below)
* [maltego-trx](https://github.com/paterva/maltego-trx) (it will be automatically installed)
* [python \>= 3](https://www.python.org/)
* [An Intelligence X API Key](https://intelx.io/account?tab=developer)

While the installation procedure is relatively straightforward, there are a few fundamental differences between Linux / Windows. Mainly, the location of the Python executable. The first step, is to ensure intelx-0.4 and maltego-trx are a part of your Python environment.

```
git clone https://github.com/IntelligenceX/SDK
pip install ./SDK/Python
```

Next, the folder will be copied and the requirements will be installed:

```
copy "./SDK/Maltego Transform" C:\intelx-maltego
cd C:\intelx-maltego
pip install -r requirements.txt
```

Next, the actual installation script can be started. In order to do that, simply run the install.py script, and follow the instructions.

```
C:\intelx-maltego>python install.py
Python executable: C:\Program Files (x86)\Python38-32\Python.exe
Intelligence X API Key: XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX

Configuration file saved to: C:\intelx-maltego/intelx.mtz
Head to Maltego > Import/Export > Import Config and select the generated file.
```

The Maltego MTZ configuration file will be automatically generated, which is required to import all of the transforms hosted in this repository. It will be located within the intelx-maltego folder. 

The final step is to import that file in Maltego by going to Maltego > Import / Export > Import Configuration > Import intelx.mtz file

You should be met with a screen similar to this:

![](https://camo.githubusercontent.com/5e51005ed2eaf24bfa35068557a7f7a8fac833ee/68747470733a2f2f692e696d6775722e636f6d2f3658474b4b72752e706e67)


## Uninstallation

If you would like to remove the entities and transforms from your Maltego installation, you must do so manually.

### Entity Removal

To remove the Intelligence X entities, simply navigate to Entities > Manage Entities > Search for "intelx", and click the "X" on the entities to remove.

![](https://i.imgur.com/5xpoXbr.png)

### Transform Removal

To remove the Intelligence X transforms, simply navigate to Transforms > Transform Manager, then search for "Intelligence" and select all transforms, then right click > Delete.

![](https://i.imgur.com/dkWbq1Q.png)

From there, all you have to do is remove the intelx-maltego directory, and you should be good. Alternatively, you can do a factory reset, and remove all entities + transforms automatically.

![](https://i.imgur.com/ze6nDkm.png)


## Updating the transforms

At the time of writing, there is not an automatic update feature. If you need to update the code, simply remove the existing transforms, entities and transform bindings, and start the installation again.

## Legal

Maltego is a trademark owned by Maltego Technologies GmbH.

The Terms of Service https://intelx.io/terms-of-service apply.

&copy; 2020 Intelligence X
