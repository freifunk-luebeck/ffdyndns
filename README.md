# FFdynDNS

Freifunk dynamic DNS Service

# Ideen

Ich würde das ganze gernn so gestalten, dass keine Accounts notwendig sind um subdomains
zu registrieren und zu updaten. Der Grund ist, dass wir so so gut wie keine personalisierten
Daten speichern müssen. Ein paar Stichpunkte:

- Kein Account notwendig
- Beim registrieren wird ein Token generiert
- Zum updaten muss das Token mitgeschickt werden
	- so wissen wir, dass der Client berechtigt ist, die Domain zu updaten
- Jede Domain hat einen Counter der initial auf 90 Tage gesetzt ist
	- Letsencrypt Zertifikate sind auch für 90 Tage gültig
- Der Counter zählt linear runter
- Mit jedem update wird der Counter wieder auf 90 Tage gesetz
- Wenn der Counter 0 erreicht wird die Domain wieder freigegeben
	- Token wird gelöscht
	- Domain kann wieder registriert werden

Um Spam zu vermeiden sollten wir ein captcha in irgendeiner Form nutzen.


Über das Token könnte man auch zusätzliche Dinge realisieren, die Authentifizierung benötigen.
Z.B. ein simples Webinterface um einzusehen, wann die Domain zuletzt geupdatet wurde oder um sie
manuell freizugeben.
