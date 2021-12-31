# FFdynDNS
[![pipeline status](https://git.chaotikum.org/freifunk-luebeck/ffdyndns/badges/master/pipeline.svg)](https://git.chaotikum.org/freifunk-luebeck/ffdyndns/-/pipelines/latest)

Current debian package [ffdyndns.deb](https://freifunk-luebeck.pages.chaotikum.org/ffdyndns/ffdyndns.deb)

Freifunk dynamic DNS Service

# Nginx config

Webserver must set `X-Forwarded-For` header. Otherwise ffdyndns cannont know the ip address of the client.


# DNS config

An example bind config:

```
zone "example.com" {
        type master;
        file "example.com";
        allow-update {
             127.0.0.1 ;
        };
};
```

And a zonefile for that zone:

```
$ORIGIN .
$TTL 30 ; 30 seconds
example.com         IN SOA  ns.example.com. hostmaster.example.com. (
                                2016062805 ; serial
                                3600       ; refresh (1 hour)
                                600        ; retry (10 minutes)
                                2600       ; expire (43 minutes 20 seconds)
                                30         ; minimum (30 seconds)
                                )
                        NS      ns.example.com.
                        NS      ns2.example.com.
```
