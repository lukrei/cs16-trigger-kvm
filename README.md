# cs16-trigger-kvm
Fork from https://github.com/rmccrystal/kvm-csgo-cheat

Das ist ein Fork von dem Github Repository https://github.com/rmccrystal/
Der Triggerbot für Counter-Strike 1.6 funktioniert mit der folgenden CS Steam Version:

Protocol version 48
Exe version 1.1.2.7/Stdio (cstrike)
Exe build: 19:53:27 Aug  3 2020 (8684)

Getestet unter Debian 11 mit KVM Hypervisor und einer Windows 10 LTSC 2019 VM mit GPU Passthrough.
Neuere Windows 10 Versionen wie 21h2 funktionieren nicht.

Der Triggerbot für CS 1.6 ist lediglich zu Demonstrationszwecken gemacht und schiesst auf Verbündete und Feinde gleichermaßen.
Man kann relativ einfach ohne tiefergehende Programmierkentnisse einen Hack kopieren und für andere Games verfügbar machen.

Der Hack wird unter Debian ausgeführt während CS 1.6 am KVM Win10 Gast läuft. Debian liest den Speicher vom Linux Gast ohne Probleme aus.
Probleme gibt es wenn CS 1.6 mehrmals gestartet wird dann injected der Hack nicht richtig. Es muss dann Steam beendet und neugestartet werden damit der Hack wieder funktioniert.

Todo List: 
Portieren auf neueres Memflow so wie zb. das Projekt: https://raw.githubusercontent.com/ntegan/memflow-hypervisor_bhop/main/bhop/src/main.rs
