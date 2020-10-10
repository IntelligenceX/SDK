#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import zipfile, json, os

tform_path = os.getcwd() + '/config/TransformRepositories/Local'
cwd = os.getcwd()

config = {

    'ixemails': {
        'filename': 'intelx.ixemails.transformsettings',
        'propval': f'{cwd}/project.py local ixemails'
    },

    'ixsubdomains': {
        'filename': 'intelx.ixsubdomains.transformsettings',
        'propval': f'{cwd}/project.py local ixsubdomains'
    },

    'ixurls': {
        'filename': 'intelx.ixurls.transformsettings',
        'propval': f'{cwd}/project.py local ixurls'
    },

    'searchdomain': {
        'filename': 'intelx.searchdomain.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchemail': {
        'filename': 'intelx.searchemail.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchurl': {
        'filename': 'intelx.searchurl.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'history': {
        'filename': 'intelx.history.transformsettings',
        'propval': f'{cwd}/project.py local ixhistory'
    },

    'fetchsearchresult': {
        'filename': 'intelx.fetchsearchresult.transformsettings',
        'propval': f'{cwd}/project.py local ixsearchresult'
    },

    'fetchhistoricalsearchresult': {
        'filename': 'intelx.fetchhistoricalsearchresult.transformsettings',
        'propval': f'{cwd}/project.py local ixsearchresult'
    },

    'treeview': {
        'filename': 'intelx.treeview.transformsettings',
        'propval': f'{cwd}/project.py local ixtreeview'
    },

    'leaktreeview': {
        'filename': 'intelx.leaktreeview.transformsettings',
        'propval': f'{cwd}/project.py local ixtreeview'
    },

    'searchip': {
        'filename': 'intelx.searchip.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchbtc': {
        'filename': 'intelx.searchbtc.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchmac': {
        'filename': 'intelx.searchmac.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchuuid': {
        'filename': 'intelx.searchuuid.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchstorageid': {
        'filename': 'intelx.searchstorageid.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchsystemid': {
        'filename': 'intelx.searchsystemid.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    },

    'searchsimhash': {
        'filename': 'intelx.searchsimhash.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch' 
    },

    'searchcreditcard': {
        'filename': 'intelx.searchcreditcard.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch' 
    },

    'searchiban': {
        'filename': 'intelx.searchiban.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch' 
    },

    'searchleak': {
        'filename': 'intelx.searchleak.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch' 
    },

    'ixselectors': {
        'filename': 'intelx.ixselectors.transformsettings',
        'propval': f'{cwd}/project.py local ixselectors'
    },

    'searchselector': {
        'filename': 'intelx.searchselector.transformsettings',
        'propval': f'{cwd}/project.py local ixsearch'
    }

}

def zip_dir(directory, zipname):
    if os.path.exists(directory):
        zf = zipfile.ZipFile(zipname, 'w', zipfile.ZIP_DEFLATED)
        root = os.path.basename(directory)
        for dirpath, dirnames, filenames in os.walk(directory):
            for filename in filenames:
                filepath   = os.path.join(dirpath, filename)
                parentpath = os.path.relpath(filepath, directory)
                arcname    = os.path.join(root, parentpath).strip("config/")
                zf.write(filepath, arcname)
    zf.close()

try:

    python_location = input("Python executable: ")
    apikey = input('Intelligence X API Key: ')

    for entry in config:
        with open(f"{tform_path}/{config[entry]['filename']}", "r+") as handle:
            contents = handle.read()
            new_contents = contents.replace("PYTHONEXEC", python_location)
            new_contents = new_contents.replace("COMMANDLINE", config[entry]['propval'])
            new_contents = new_contents.replace("WORKINGDIR", cwd)
            handle.seek(0)
            handle.write(new_contents)
            handle.truncate()

    with open(f'{cwd}/settings.json', 'w') as handle:
        settings = {
            "APIKEY": apikey,
            "PYTHONEXEC": python_location,
            "WORKINGDIR": cwd,
        }
        handle.write(json.dumps(settings))
        handle.close()

    zip_dir(f"{cwd}/config", "intelx.mtz")
    
    print(f"\nConfiguration file saved to: {cwd}/intelx.mtz")
    print("Head to Maltego > Import/Export > Import Config and select the generated file.")

except Exception as e:
    print(e)