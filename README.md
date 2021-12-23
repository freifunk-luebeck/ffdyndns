# FFdynDNS
[![pipeline status](https://git.chaotikum.org/paul/ffdyndns/badges/master/pipeline.svg)](https://git.chaotikum.org/paul/ffddns/-/commits/master)

Current debian package [ffdyndns.deb](https://freifunk-luebeck.pages.chaotikum.org/ffdyndns/ffdyndns.deb)

Freifunk dynamic DNS Service

# Ideen

Ich würde das ganze gernn so gestalten, dass keine Accounts notwendig sind um subdomains
zu registrieren und zu updaten. Der Grund ist, dass wir so so gut wie keine personalisierten
Daten speichern müssen.



- Beim registrieren einer Domain wird ein Token generiert
- Beim update über http-api muss das Token mitgeschickt werden
	- so wissen wir, dass der Client berechtigt ist, die Domain zu updaten
- Jede Domain hat einen Counter der initial auf 90 Tage gesetzt ist
	- Letsencrypt Zertifikate sind auch für 90 Tage gültig
- Der Counter zählt runter
- Mit jedem update wird der Counter wieder auf 90 Tage gesetz
- Wenn der Counter 0 erreicht wird die Domain wieder freigegeben
	- Token wird gelöscht
	- Domain kann wieder registriert werden

Über das Token könnte man auch zusätzliche Dinge realisieren, die Authentifizierung benötigen.
Z.B. ein simples Webinterface um einzusehen, wann die Domain zuletzt geupdatet wurde oder um sie
manuell freizugeben.
