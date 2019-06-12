locator
========

locate intelligence service(s) that has the sample

usage
------

```sh
# for example..
$ curl https://xxx.com/something/includes/iocs | IoCs hashes | locator
hash    virustotal      alienvault otx  virusbay        reverse.it      malshare
001cf7af29382f4f784fe45df131ca9e14908c6c0717899780f9354b8a5f0090        true    true    false   true    false
03ff895c99555f00792a41e3b014f16ef6b4bb0c74d1fa2237a6a9275e2b2109        true    true    false   true    false
04078ef95a70a04e95bda06cc7bec3fa        false   true    false   false   false
061089d8cb0ca58e660ce2e433a689b3        false   true    false   false   false
# ...
```

* required trailing apikeys(and set them into each envvar)

|service|envvar|
|:--|:--|
|VirusTotal|`$VTAPIKEY`|
|AlienVault OTX|`$OTX_APIKEY`|
|reverse.it|`$REVIT_APIKEY`|
|MalShare|`$MALSHARE_APIKEY`|

setup
------

* Rust 1.35.0

```sh
cargo install --git https://github.com/0x75960/locator
```

